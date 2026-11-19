import type { ThemeConfig } from '@/api/types'

export interface Rgb {
  r: number
  g: number
  b: number
}

export function hexToRgb(value: string): Rgb | null {
  const hex = value.replace('#', '').trim()
  if (![3, 4, 6, 8].includes(hex.length) || !/^[0-9a-f]+$/i.test(hex)) return null
  const normalized =
    hex.length === 3 || hex.length === 4
      ? hex
          .slice(0, 3)
          .split('')
          .map((char) => char + char)
          .join('')
      : hex.slice(0, 6)
  return {
    r: parseInt(normalized.slice(0, 2), 16),
    g: parseInt(normalized.slice(2, 4), 16),
    b: parseInt(normalized.slice(4, 6), 16),
  }
}

export function luminance(value: string): number {
  const rgb = hexToRgb(value)
  if (!rgb) return 0
  const channel = (component: number) => {
    const normalized = component / 255
    return normalized <= 0.03928 ? normalized / 12.92 : ((normalized + 0.055) / 1.055) ** 2.4
  }
  return 0.2126 * channel(rgb.r) + 0.7152 * channel(rgb.g) + 0.0722 * channel(rgb.b)
}

export function isLightTheme(theme: ThemeConfig): boolean {
  return luminance(theme.colors.background) > 0.55
}

export function readableOn(value: string): string {
  return luminance(value) > 0.5 ? '#101014' : '#F5F5FA'
}

export function withAlpha(value: string, alpha: number): string {
  const rgb = hexToRgb(value)
  if (!rgb) return value
  return `rgba(${rgb.r}, ${rgb.g}, ${rgb.b}, ${Math.max(0, Math.min(1, alpha))})`
}
