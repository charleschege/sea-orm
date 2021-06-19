use crate::{EntityName, IdenStatic, Iterable};
use sea_query::{DynIden, Expr, SeaRc, SimpleExpr, Value};

#[derive(Debug, Clone)]
pub struct ColumnDef {
    pub(crate) col_type: ColumnType,
    pub(crate) null: bool,
    pub(crate) unique: bool,
    pub(crate) indexed: bool,
}

#[derive(Debug, Clone)]
pub enum ColumnType {
    Char(Option<u32>),
    String(Option<u32>),
    Text,
    TinyInteger,
    SmallInteger,
    Integer,
    BigInteger,
    Float,
    Double,
    Decimal(Option<(u32, u32)>),
    DateTime,
    Timestamp,
    Time,
    Date,
    Binary,
    Boolean,
    Money(Option<(u32, u32)>),
    Json,
    JsonBinary,
    Custom(String),
}

macro_rules! bind_oper {
    ( $op: ident ) => {
        fn $op<V>(&self, v: V) -> SimpleExpr
        where
            V: Into<Value>,
        {
            Expr::tbl(self.entity_name(), *self).$op(v)
        }
    };
}

macro_rules! bind_agg_func {
    ( $func: ident ) => {
        fn $func(&self) -> SimpleExpr {
            Expr::tbl(self.entity_name(), *self).$func()
        }
    };
}

macro_rules! bind_vec_func {
    ( $func: ident ) => {
        #[allow(clippy::wrong_self_convention)]
        fn $func<V, I>(&self, v: I) -> SimpleExpr
        where
            V: Into<Value>,
            I: IntoIterator<Item = V>,
        {
            Expr::tbl(self.entity_name(), *self).$func(v)
        }
    };
}

// LINT: when the operand value does not match column type
pub trait ColumnTrait: IdenStatic + Iterable {
    type EntityName: EntityName;

    fn def(&self) -> ColumnDef;

    fn entity_name(&self) -> DynIden {
        SeaRc::new(Self::EntityName::default()) as DynIden
    }

    fn as_column_ref(&self) -> (DynIden, DynIden) {
        (self.entity_name(), SeaRc::new(*self) as DynIden)
    }

    bind_oper!(eq);
    bind_oper!(ne);
    bind_oper!(gt);
    bind_oper!(gte);
    bind_oper!(lt);
    bind_oper!(lte);

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.between(2,3))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` BETWEEN 2 AND 3"
    /// );
    /// ```
    fn between<V>(&self, a: V, b: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::tbl(self.entity_name(), *self).between(a, b)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Id.not_between(2,3))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`id` NOT BETWEEN 2 AND 3"
    /// );
    /// ```
    fn not_between<V>(&self, a: V, b: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::tbl(self.entity_name(), *self).not_between(a, b)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.like("cheese"))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE 'cheese'"
    /// );
    /// ```
    fn like(&self, s: &str) -> SimpleExpr {
        Expr::tbl(self.entity_name(), *self).like(s)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.not_like("cheese"))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` NOT LIKE 'cheese'"
    /// );
    /// ```
    fn not_like(&self, s: &str) -> SimpleExpr {
        Expr::tbl(self.entity_name(), *self).not_like(s)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.starts_with("cheese"))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE 'cheese%'"
    /// );
    /// ```
    fn starts_with(&self, s: &str) -> SimpleExpr {
        let pattern = format!("{}%", s);
        Expr::tbl(self.entity_name(), *self).like(&pattern)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.ends_with("cheese"))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese'"
    /// );
    /// ```
    fn ends_with(&self, s: &str) -> SimpleExpr {
        let pattern = format!("%{}", s);
        Expr::tbl(self.entity_name(), *self).like(&pattern)
    }

    /// ```
    /// use sea_orm::{entity::*, query::*, tests_cfg::cake, sea_query::MysqlQueryBuilder};
    ///
    /// assert_eq!(
    ///     cake::Entity::find()
    ///         .filter(cake::Column::Name.contains("cheese"))
    ///         .build(MysqlQueryBuilder)
    ///         .to_string(),
    ///     "SELECT `cake`.`id`, `cake`.`name` FROM `cake` WHERE `cake`.`name` LIKE '%cheese%'"
    /// );
    /// ```
    fn contains(&self, s: &str) -> SimpleExpr {
        let pattern = format!("%{}%", s);
        Expr::tbl(self.entity_name(), *self).like(&pattern)
    }

    bind_agg_func!(max);
    bind_agg_func!(min);
    bind_agg_func!(sum);
    bind_agg_func!(count);

    fn if_null<V>(&self, v: V) -> SimpleExpr
    where
        V: Into<Value>,
    {
        Expr::tbl(self.entity_name(), *self).if_null(v)
    }

    bind_vec_func!(is_in);
    bind_vec_func!(is_not_in);
}

impl ColumnType {
    pub fn def(self) -> ColumnDef {
        ColumnDef {
            col_type: self,
            null: false,
            unique: false,
            indexed: false,
        }
    }
}

impl ColumnDef {
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    pub fn null(mut self) -> Self {
        self.null = true;
        self
    }

    pub fn indexed(mut self) -> Self {
        self.indexed = true;
        self
    }
}

impl Into<sea_query::ColumnType> for ColumnType {
    fn into(self) -> sea_query::ColumnType {
        match self {
            Self::Char(s) => sea_query::ColumnType::Char(s),
            Self::String(s) => sea_query::ColumnType::String(s),
            Self::Text => sea_query::ColumnType::Text,
            Self::TinyInteger => sea_query::ColumnType::TinyInteger(None),
            Self::SmallInteger => sea_query::ColumnType::SmallInteger(None),
            Self::Integer => sea_query::ColumnType::Integer(None),
            Self::BigInteger => sea_query::ColumnType::BigInteger(None),
            Self::Float => sea_query::ColumnType::Float(None),
            Self::Double => sea_query::ColumnType::Double(None),
            Self::Decimal(s) => sea_query::ColumnType::Decimal(s),
            Self::DateTime => sea_query::ColumnType::DateTime(None),
            Self::Timestamp => sea_query::ColumnType::Timestamp(None),
            Self::Time => sea_query::ColumnType::Time(None),
            Self::Date => sea_query::ColumnType::Date,
            Self::Binary => sea_query::ColumnType::Binary(None),
            Self::Boolean => sea_query::ColumnType::Boolean,
            Self::Money(s) => sea_query::ColumnType::Money(s),
            Self::Json => sea_query::ColumnType::Json,
            Self::JsonBinary => sea_query::ColumnType::JsonBinary,
            Self::Custom(s) => {
                sea_query::ColumnType::Custom(sea_query::SeaRc::new(sea_query::Alias::new(&s)))
            }
        }
    }
}

impl From<sea_query::ColumnType> for ColumnType {
    fn from(col_type: sea_query::ColumnType) -> Self {
        match col_type {
            sea_query::ColumnType::Char(s) => Self::Char(s),
            sea_query::ColumnType::String(s) => Self::String(s),
            sea_query::ColumnType::Text => Self::Text,
            sea_query::ColumnType::TinyInteger(_) => Self::TinyInteger,
            sea_query::ColumnType::SmallInteger(_) => Self::SmallInteger,
            sea_query::ColumnType::Integer(_) => Self::Integer,
            sea_query::ColumnType::BigInteger(_) => Self::BigInteger,
            sea_query::ColumnType::Float(_) => Self::Float,
            sea_query::ColumnType::Double(_) => Self::Double,
            sea_query::ColumnType::Decimal(s) => Self::Decimal(s),
            sea_query::ColumnType::DateTime(_) => Self::DateTime,
            sea_query::ColumnType::Timestamp(_) => Self::Timestamp,
            sea_query::ColumnType::Time(_) => Self::Time,
            sea_query::ColumnType::Date => Self::Date,
            sea_query::ColumnType::Binary(_) => Self::Binary,
            sea_query::ColumnType::Boolean => Self::Boolean,
            sea_query::ColumnType::Money(s) => Self::Money(s),
            sea_query::ColumnType::Json => Self::Json,
            sea_query::ColumnType::JsonBinary => Self::JsonBinary,
            sea_query::ColumnType::Custom(s) => Self::Custom(s.to_string()),
        }
    }
}
