from .. utils import TranspileTestCase, BuiltinFunctionTestCase


class BoolTests(TranspileTestCase):
    pass


class BuiltinBoolFunctionTests(BuiltinFunctionTestCase, TranspileTestCase):
    functions = ["bool"]

    not_implemented = [
        'test_bytearray',
        'test_class',
        'test_complex',
        'test_class',
        'test_dict',
        'test_frozenset',
        'test_list',
        'test_range',
        'test_set',
    ]
