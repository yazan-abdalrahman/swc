//// [emitExponentiationOperatorInTempalteString4.ts]
import _type_of from "@swc/helpers/src/_type_of.mjs";
var t1 = 10;
++t1, t1++, --t1, _type_of(Math.pow(t1, Math.pow(10, t1))), "".concat(Math.pow(t1, -10), " hello world ").concat(Math.pow(t1, -10)), "".concat(Math.pow(-t1, 10) - t1, " hello world ").concat(Math.pow(-t1, 10) - t1), "".concat(Math.pow(-++t1, 10) - t1, " hello world ").concat(Math.pow(t1, Math.pow(-++t1, -t1))), "".concat(Math.pow(-t1++, 10) - t1, " hello world ").concat(Math.pow(10, Math.pow(-t1++, -t1))), "".concat(Math.pow(~t1, Math.pow(10, --t1)), " hello world ").concat(Math.pow(~t1, Math.pow(10, --t1))), "".concat(_type_of(Math.pow(t1, Math.pow(10, t1))), " hello world ").concat(_type_of(Math.pow(t1, Math.pow(10, t1)))), ++t1, t1++, --t1, _type_of(Math.pow(t1, Math.pow(10, t1)));
