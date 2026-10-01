import { createServer, type IncomingMessage, type Server } from 'node:http'
import type { AddressInfo } from 'node:net'
import { afterAll, beforeAll, describe, expect, inject, it } from 'vitest'
import { harness } from '../src/sdk'

interface Recorded {
  method: string
  url: string
  authorization?: string
}

const bin = inject('harness')
const API_TOKEN = 'functional-test-token'

let server: Server
let base = ''
const requests: Recorded[] = []
const responses = new Map<string, unknown>()

function record(req: IncomingMessage): Recorded {
  const entry = { method: req.method ?? '', url: req.url ?? '', authorization: req.headers.authorization }
  requests.push(entry)
  return entry
}

beforeAll(async () => {
  server = createServer((req, res) => {
    const entry = record(req)
    const body = responses.get(`${entry.method} ${entry.url}`)
    if (body === undefined) {
      res.writeHead(404, { 'Content-Type': 'application/json' })
      res.end(JSON.stringify({ detail: 'not found' }))
      return
    }
    res.writeHead(200, { 'Content-Type': 'application/json' })
    res.end(JSON.stringify(body))
  })
  await new Promise<void>((resolve) => server.listen(0, '127.0.0.1', resolve))
  base = `http://127.0.0.1:${(server.address() as AddressInfo).port}`
})

afterAll(async () => {
  await new Promise<void>((resolve) => server.close(() => resolve()))
})

function call(fn: string, arg: string) {
  return harness(bin, [fn, base, API_TOKEN, arg])
}

function lastRequest(): Recorded {
  return requests[requests.length - 1]
}

describe('HTTP round trip against a local API double', () => {
  it('personal_tokens: GET by id with path parameter and token auth', async () => {
    const id = '2c5a2e1e-6c5b-4b5e-9b0e-3f6d2f9a1b7c'
    // PersonalTokenResponseGet is a flat object in this crate's OpenAPI document.
    responses.set(`GET /iam/personal_tokens/${id}`, {
      uuid: id,
      name: 'ci-token',
      created: '2026-01-01T00:00:00Z',
      expires_at: '2027-01-01T00:00:00Z',
      description: null,
    })

    const result = await call('personal_tokens.get_personal_token', id)

    expect(lastRequest()).toMatchObject({ method: 'GET', url: `/iam/personal_tokens/${id}`, authorization: `Token ${API_TOKEN}` })
    expect(result.ok).toBe(true)
    expect(result.data).toMatchObject({ uuid: id, name: 'ci-token', expires_at: '2027-01-01T00:00:00Z' })
  })

  it('variables: GET by uuid deserializes the Variable model', async () => {
    const uuid = '0b6c2a52-3d6f-4f7a-9a43-6a2d0c1e9f10'
    responses.set(`GET /variables/${uuid}`, {
      uuid,
      key: 'API_URL',
      value: 'https://example.com',
      secret: false,
      last_editor: 'ci@example.com',
      created_at: '2026-01-01T00:00:00Z',
      updated_at: '2026-01-02T00:00:00Z',
    })

    const result = await call('variables.api_variables_retrieve', uuid)

    expect(lastRequest()).toMatchObject({ method: 'GET', url: `/variables/${uuid}`, authorization: `Token ${API_TOKEN}` })
    expect(result).toMatchObject({ ok: true, data: { uuid, key: 'API_URL', value: 'https://example.com', secret: false } })
  })

  it('edgefunctions (reqwest 0.12 crate): GET by numeric id deserializes the nested results', async () => {
    responses.set('GET /edge_functions/42', {
      results: { id: 42, name: 'hello-world', language: 'javascript', active: true },
      schema_version: 3,
    })

    const result = await call('edgefunctions.edge_functions_id_get', '42')

    expect(lastRequest()).toMatchObject({ method: 'GET', url: '/edge_functions/42', authorization: `Token ${API_TOKEN}` })
    expect(result.ok).toBe(true)
    expect(result.data).toMatchObject({ results: { id: 42, name: 'hello-world' }, schema_version: 3 })
  })

  it('domains: GET by id deserializes the domain entity', async () => {
    responses.set('GET /domains/7', {
      results: { id: 7, name: 'site', cnames: ['www.example.com'], is_active: true, domain_name: 'abc.map.azionedge.net' },
      schema_version: 3,
    })

    const result = await call('domains.get_domain', '7')

    expect(lastRequest()).toMatchObject({ method: 'GET', url: '/domains/7', authorization: `Token ${API_TOKEN}` })
    expect(result.ok).toBe(true)
    expect(result.data.results).toMatchObject({ id: 7, cnames: ['www.example.com'], domain_name: 'abc.map.azionedge.net' })
  })

  it('surfaces HTTP errors as SDK response errors with the status code', async () => {
    const result = await call('variables.api_variables_retrieve', 'missing')

    expect(lastRequest()).toMatchObject({ method: 'GET', url: '/variables/missing' })
    expect(result).toMatchObject({ ok: false, status: 404 })
  })
})
