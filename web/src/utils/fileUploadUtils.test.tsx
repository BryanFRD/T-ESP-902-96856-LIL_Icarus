import { describe, expect, it, vi } from 'vitest'

import {
  formatFileSize,
  getDisplaySize,
  getFileIcon,
  getPreview,
  processErrors
} from './fileUploadUtils'

describe('fileUploadUtils', () => {
  it('getFileIcon returns Image for image/*', () => {
    const file = { type: 'image/png' } as File
    const icon = getFileIcon(file)
    expect(icon).toBeDefined()
    expect((icon as React.ReactElement).type).toBeDefined()
    const typeName =
      (
        (icon as React.ReactElement).type as {
          name?: string
          displayName?: string
        }
      ).name ||
      ((icon as React.ReactElement).type as { displayName?: string })
        .displayName
    expect(typeName).toMatch(/Image/)
  })

  it('getFileIcon returns FileText for pdf', () => {
    const file = { type: 'application/pdf' } as File
    const icon = getFileIcon(file)
    expect(icon).toBeDefined()
    expect((icon as React.ReactElement).type).toBeDefined()
    const typeName =
      (
        (icon as React.ReactElement).type as {
          name?: string
          displayName?: string
        }
      ).name ||
      ((icon as React.ReactElement).type as { displayName?: string })
        .displayName
    expect(typeName).toMatch(/FileText/)
  })

  it('getFileIcon returns File for other types', () => {
    const file = { type: 'application/zip' } as File
    const icon = getFileIcon(file)
    expect(icon).toBeDefined()
    expect((icon as React.ReactElement).type).toBeDefined()
    const typeName =
      (
        (icon as React.ReactElement).type as {
          name?: string
          displayName?: string
        }
      ).name ||
      ((icon as React.ReactElement).type as { displayName?: string })
        .displayName
    expect(typeName).toMatch(/File/)
  })

  it('getPreview returns url for image/*', () => {
    const file = new File([''], 'test.png', { type: 'image/png' })
    const url = getPreview(file)
    expect(url).toMatch(/^blob:/)
    URL.revokeObjectURL(url!)
  })

  it('getPreview returns null for non-image', () => {
    const file = new File([''], 'test.pdf', { type: 'application/pdf' })
    expect(getPreview(file)).toBeNull()
  })

  it('formatFileSize formats bytes', () => {
    expect(formatFileSize(0)).toBe('0 B')
    expect(formatFileSize(1024)).toBe('1 KB')
    expect(formatFileSize(1048576)).toBe('1 MB')
    expect(formatFileSize(1073741824)).toBe('1 GB')
  })

  vi.mock('i18next', () => ({
    t: (key: string, opts: unknown) => `${key}:${JSON.stringify(opts)}`
  }))

  it('processErrors returns correct error messages', () => {
    type FileError = { code: string }
    type FileRejection = { file: { name: string }; errors: FileError[] }
    const rejectedFiles: FileRejection[] = [
      {
        file: { name: 'big.png' },
        errors: [{ code: 'file-too-large' }]
      },
      {
        file: { name: 'bad.pdf' },
        errors: [{ code: 'file-invalid-type' }]
      },
      {
        file: { name: 'extra.zip' },
        errors: [{ code: 'too-many-files' }]
      }
    ]
    const errors = processErrors(
      rejectedFiles as unknown as import('react-dropzone').FileRejection[],
      2,
      1,
      ['image/png']
    )
    expect(errors[0]).toMatch(/file_upload.errors.too_large/)
    expect(errors[1]).toMatch(/file_upload.errors.invalid_type/)
    expect(errors[2]).toMatch(/file_upload.errors.too_many_files/)
  })

  it('getDisplaySize returns formatted string', () => {
    expect(getDisplaySize(2)).toBe(' (max 2MB)')
  })
})
