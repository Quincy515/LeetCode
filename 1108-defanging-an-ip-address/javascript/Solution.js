/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function(address) {
    return address.replaceAll('.','[.]')
};

/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function(address) {
    let tempAddress = ''
    for (let i=0; i<address.length; i++) {
        if (address[i] != '.') {
            tempAddress += address[i]
        } else {
            tempAddress += '[.]'
        }
    }
    return tempAddress
}

/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function(address) {
    let origin = [...address]
    let n = origin.length
    let newN = n+2*3
    let newString = new Array(newN)
    let k = 0
    for (let i = 0; i < n; i++) {
        if (origin[i] != '.') {
            newString[k] = origin[i]
            k++
        } else {
            newString[k++] = '['
            newString[k++] = '.'
            newString[k++] = ']'
        }
    }
    return newString.join('')
}

