pub trait DbCmp<'a> {
    type DbType;
    fn compare_to_db(self, rhs: Self::DbType) -> Option<Vec<Self::DbType>>;
}
