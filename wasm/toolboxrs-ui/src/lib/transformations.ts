//@ts-ignore
import * as wasmModule from 'wasm'
import type { ActionInput, CidrOutput } from "./types";


export const reducer = (actionInput: ActionInput): string => {

    const { action, inputValue } = actionInput
    try {
        switch (action.type) {
            case 'base64':
                // use javascript native functions
                return action.subOption == 'encode'
                    ? btoa(inputValue)
                    : atob(inputValue)
            case 'cidr':
                const cidrOutput: CidrOutput = wasmModule.cidr(inputValue)
                if (cidrOutput.message == '') {
                    const outputString = `
network_length: <b>${cidrOutput.network_length}</b>
cidr_range: <b>${cidrOutput.cidr_range}</b>
subnet_mask: <b>${cidrOutput.subnet_mask}</b>
first_ip: <b>${cidrOutput.first_ip}</b>
last_ip: <b>${cidrOutput.last_ip}</b>
number of ips in this cidr range: <b>${cidrOutput.num_of_ips}</b>
`
                    return outputString
                } else {
                    return cidrOutput.message
                }

            case 'hash':
                return wasmModule.hash(inputValue, action.subOption!)
            default:
                return 'invalid input, try again'
        }
    } catch (e) {
        if (typeof e === 'string') {
            console.error(e)
            return 'no valid output'
        } else if (e instanceof Error) {
            console.error(e.message)
            return 'no valid output'
        } else {
            console.error('could not parse error')
            return 'no valid output'
        }
    }
}
