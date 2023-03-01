TYPES = """
i8 i16 i32 i64 isize
u8 u16 u32 u64 usize
""".strip().split()


for t in TYPES:
    print('''
impl cmp::PartialEq<BoolIsh> for %(t)s {
    fn eq(&self, other: &BoolIsh) -> bool {
        i64_fuzzy_eq(other.value, *self as i64)
    }
}'''.strip() % {'t': t})
    print()