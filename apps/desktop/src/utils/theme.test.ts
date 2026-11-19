import { describe, expect, it } from 'vitest'
import type { ThemeConfig } from '@/api/types'
import { hexToRgb, isLightTheme, luminance, readableOn, withAlpha } from './theme'

function theme(background: string): ThemeConfig {
  return {
    version: 1,
    name: 'Test',
    preset: 'custom',
    colors: {
      primary: '#7C4DFF',
      secondary: '#00E5FF',
      surface: '#121218',
      background,
      error: '#FF5370',
      success: '#4ADE80',
      info: '#38BDF8',
      warning: '#FBBF24',
    },
    typography: { fontFamily: 'sans-serif', scale: 1 },
    background: { mode: 'cover', opacity: 0.3, blur: 8, image: null },
    effects: { rounded: 12, shadows: true, motion: true },
  }
}

describe('theme utilities', () => {
  it('parses short and long hex colors', () => {
    expect(hexToRgb('#fff')).toEqual({ r: 255, g: 255, b: 255 })
    expect(hexToRgb('#7C4DFF')).toEqual({ r: 124, g: 77, b: 255 })
    expect(hexToRgb('nope')).toBeNull()
  })

  it('computes relative luminance ordering', () => {
    expect(luminance('#000000')).toBeLessThan(luminance('#FFFFFF'))
    expect(luminance('#808080')).toBeGreaterThan(0.1)
  })

  it('detects light themes from background', () => {
    expect(isLightTheme(theme('#F6F4FF'))).toBe(true)
    expect(isLightTheme(theme('#0B0B0F'))).toBe(false)
  })

  it('picks readable foreground colors', () => {
    expect(readableOn('#FFFFFF')).toBe('#101014')
    expect(readableOn('#000000')).toBe('#F5F5FA')
  })

  it('applies alpha to hex colors', () => {
    expect(withAlpha('#7C4DFF', 0.5)).toBe('rgba(124, 77, 255, 0.5)')
  })
})
