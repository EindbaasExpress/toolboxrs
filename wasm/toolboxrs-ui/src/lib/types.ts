export interface OptionsSet {
    value: string
    label: string
    suboptions?: OptionsSet[]
}

export interface ActionInput {
    action: {
        type: string | undefined
        subOption?: string
    }
    inputValue: string
}

export interface CidrOutputValid {
    network_length: string
    cidr_range: string
    subnet_mask: string
    first_ip: string
    last_ip: string
    num_of_ips: string
    message: ''
}
export interface CidrOutputInvalid {
    network_length: ''
    cidr_range: ''
    subnet_mask: ''
    first_ip: ''
    last_ip: ''
    num_of_ips: ''
    message: string
}
export type CidrOutput = CidrOutputValid | CidrOutputInvalid