from .. utils import TranspileTestCase, BuiltinFunctionTestCase


class BuiltinAbsFunctionTests(BuiltinFunctionTestCase, TranspileTestCase):
    functions = ["abs"]

    not_implemented = [
        'abs',
        'test_bytearray',
        'test_bytes',
        'test_complex',
        'test_frozenset',
    ]
