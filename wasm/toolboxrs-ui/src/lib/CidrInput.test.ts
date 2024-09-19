import { fireEvent, render } from '@testing-library/svelte'
import CidrInput from './CidrInput.svelte'
import { describe, it, expect } from 'vitest'

describe('CidrInput component', () => {
    it('renders with a placeholder', () => {
        const { getByText } = render(CidrInput)
        // @ts-ignore
        expect(getByText('X.X.X.X/Y')).toBeInTheDocument()
    })

    it('renders with a green outline when the value matches the expectated format', async () => {
        const { getByRole } = render(CidrInput)
        // @ts-ignore
        const inputElement = getByRole('textbox')
        await fireEvent.input(inputElement, {
            target: { value: '10.0.0.0/20' },
        })
        const style = getComputedStyle(inputElement)

        // Validate the outline color and width
        expect(style.outline).toBe('2px solid green')
    })

    it('renders with a red outline when the value does not match the expected format', async () => {
        const { getByRole } = render(CidrInput)
        // @ts-ignore
        const inputElement = getByRole('textbox')
        await fireEvent.input(inputElement, {
            target: { value: 'this is no cidr range' },
        })
        const style = getComputedStyle(inputElement)

        // Assert that the outline is yellow
        expect(style.outline).toBe('2px solid red')
    })
    it('renders with a yellow outline when the value partially matches the expected format', async () => {
        const { getByRole } = render(CidrInput)
        // @ts-ignore
        const inputElement = getByRole('textbox')
        await fireEvent.input(inputElement, { target: { value: '10.0.0' } })
        const style = getComputedStyle(inputElement)

        // Assert that the outline is yellow
        expect(style.outline).toBe('2px solid yellow')
    })
})
