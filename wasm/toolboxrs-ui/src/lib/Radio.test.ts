import { fireEvent, render } from '@testing-library/svelte'
import Radio from './Radio.svelte'
import { describe, it, expect } from 'vitest'

describe('Radio component', () => {
    it('renders with a legend', () => {
        const { getByText } = render(Radio, {
            props: {
                options: [
                    { value: 'value1', label: 'label1' },
                    { value: 'value2', label: 'label2' },
                ],
                legend: 'someLegend',
            },
        })
        // @ts-ignore
        expect(getByText('someLegend')).toBeInTheDocument()
    })
    it('renders with options', () => {
        const { getAllByRole } = render(Radio, {
            props: {
                options: [
                    { value: 'value1', label: 'label1' },
                    { value: 'value2', label: 'label2' },
                ],
                legend: 'someLegend',
            },
        })
        // @ts-ignore
        expect(getAllByRole('radio').length).toBe(2)
    })
})
