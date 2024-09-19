interface ValidationOutput {
    indicationColor: validationOutputColor
    inputValue: string
}
enum validationOutputColor {
    FullMatch = 'green',
    PartialMatch = 'yellow',
    NoMatch = 'red',
}
/**
 *
 * @param input
 * @param noMatch
 * @returns ValidationOutput with the color to be used for user indication
 * and the validatedOutput, which is the full string when validated or an empty
 * string, to not impact the wasm package below
 */
export const validator = (
    input: string,
    noMatch: boolean
): ValidationOutput => {
    // validate input format as X.X.X.X/Y
    const formatValidation =
        /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})\/(\d{1,2})$/

    // destructure both elements and validate on functional level
    const [ip, cidrBlock] = input.split('/')
    const valueValidationIp = ip.split('.').map((x) => Number(x) <= 255)
    const valueValidationBlock = Number(cidrBlock) <= 32
    if (
        formatValidation.test(input) &&
        valueValidationIp &&
        valueValidationBlock
    ) {
        return {
            indicationColor: validationOutputColor.FullMatch,
            inputValue: input,
        }
    } else if (!noMatch && valueValidationIp) {
        return {
            indicationColor: validationOutputColor.PartialMatch,
            inputValue: '',
        }
    } else {
        return {
            indicationColor: validationOutputColor.NoMatch,
            inputValue: '',
        }
    }
}

function findMatchingFormat(formats: InputFormatCollection, input: string) {
    for (const [key, format] of Object.entries(formats)) {
        if (format.regex.test(input)) {
            return { key, format }
        }
    }
    return null // No match found
}

interface FormatterOutput {
    input: string
    guide: string | undefined
}
interface InputFormatCollection {
    [key: string]: {
        regex: RegExp
        guide: string
    }
}
export const formatHelper = (input: string): FormatterOutput => {
    const fullFormat = 'X.X.X.X/Y'

    const allowedInputFormats: InputFormatCollection = {
        X: {
            regex: /^(\d{1,3})$/,
            guide: fullFormat.substring(1),
        },
        'X.': {
            regex: /^(\d{1,3})\.$/,
            guide: fullFormat.substring(2),
        },
        'X.X': {
            regex: /^(\d{1,3})\.(\d{1,3})$/,
            guide: fullFormat.substring(3),
        },
        'X.X.': {
            regex: /^(\d{1,3})\.(\d{1,3})\.$/,
            guide: fullFormat.substring(4),
        },
        'X.X.X': {
            regex: /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/,
            guide: fullFormat.substring(5),
        },
        'X.X.X.': {
            regex: /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.$/,
            guide: fullFormat.substring(6),
        },
        'X.X.X.X': {
            regex: /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})$/,
            guide: fullFormat.substring(7),
        },
        'X.X.X.X/': {
            regex: /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})\/$/,
            guide: fullFormat.substring(8),
        },
        'X.X.X.X/Y': {
            regex: /^(\d{1,3})\.(\d{1,3})\.(\d{1,3})\.(\d{1,3})\/(\d{1,2})$/,
            guide: '',
        },
    }
    const result = findMatchingFormat(allowedInputFormats, input)
    if (result) {
        const guide = result.format.guide
        return { input, guide }
    } else {
        return { input, guide: undefined }
    }
}
