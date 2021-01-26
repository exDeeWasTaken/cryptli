const {cesar} = require("../index.js");
const assert = require('assert');



describe("Cesar Tests", function() {
    it("Encrypt", function() {
        assert.equal(cesar("aAzZ!.?`", 50), "yYxX!.?`");
    });
    it('Decrypt ', function () {
        assert.equal(cesar("yYxX!.?`", -50), "aAzZ!.?`");
    });
});