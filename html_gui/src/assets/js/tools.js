export default {
    deepClone(source) {
        //apparently this is how you do  deep clone in js in a few lines according to the MDN.
        return JSON.parse(JSON.stringify(source));
    }
}
