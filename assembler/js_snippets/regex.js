const RE_COMMON = String.raw`(?:(\$|b|lo |hi |%)?)((?:[A-f0-9]{1,4})|(?:[01]{1,16})|(?:[0-9]{1,8}|(?:[A-z_][A-z0-9_]{2,10})))`;

function execRegex(which, input, bounds) {
    let results = which.exec(input);
    let currentOffset = 0;
    let arrayI = 0;
    if (results) {
        for (let i = 1; i < results.length; i++) {
            let match = results[i];

            if (match === undefined) {
                bounds[arrayI++] = 0;
                bounds[arrayI++] = 0;
            } else {
                bounds[arrayI++] = currentOffset;
                currentOffset += match.length;
                bounds[arrayI++] = currentOffset;
            }
        }
    }

    return bounds;
}

const RE_NORMAL_ADDR = new RegExp(
    String.raw`^(?:(#)?)` +
    RE_COMMON +
    String.raw`$`
)

export function reNormalAddressing(line, container) {
    return execRegex(RE_NORMAL_ADDR, line, container);
}


const RE_INDEXED_ADDR = new RegExp(
    String.raw`^(?:(\(|\&|\*)?)` +
    RE_COMMON +
    String.raw`(?:(\))?)(?:,?([XY])?)(?:(\))?)$`
)

export function reIndexedAddressing(line, container) {
    return execRegex(RE_INDEXED_ADDR, line, container);
}

