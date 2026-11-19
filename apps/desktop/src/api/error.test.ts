import { describe, expect, it } from 'vitest'
import { errorMessage } from './index'

describe('errorMessage', () => {
  it('returns string errors as-is', () => {
    expect(errorMessage('boom')).toBe('boom')
  })

  it('formats structured errors with hints', () => {
    expect(
      errorMessage({ code: 'authenticationFailed', message: '认证失败', hint: '检查密码' }),
    ).toBe('认证失败 · 检查密码')
  })

  it('falls back to string conversion', () => {
    expect(errorMessage(42)).toBe('42')
  })
})
