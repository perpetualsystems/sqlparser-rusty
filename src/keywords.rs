// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module defines
//! 1) a list of constants for every keyword
//! 2) an `ALL_KEYWORDS` array with every keyword in it
//!     This is not a list of *reserved* keywords: some of these can be
//!     parsed as identifiers if the parser decides so. This means that
//!     new keywords can be added here without affecting the parse result.
//!
//!     As a matter of fact, most of these keywords are not used at all
//!     and could be removed.
//! 3) a `RESERVED_FOR_TABLE_ALIAS` array with keywords reserved in a
//! "table alias" context.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "visitor")]
use sqlparser_derive::{Visit, VisitMut};

/// Defines a string constant for a single keyword: `kw_def!(SELECT);`
/// expands to `pub const SELECT = "SELECT";`
macro_rules! kw_def {
    ($ident:ident = $string_keyword:expr) => {
        pub const $ident: &'static str = $string_keyword;
    };
    ($ident:ident) => {
        kw_def!($ident = stringify!($ident));
    };
}

/// Expands to a list of `kw_def!()` invocations for each keyword
/// and defines an ALL_KEYWORDS array of the defined constants.
macro_rules! define_keywords {
    ($(
        $ident:ident $(= $string_keyword:expr)?
    ),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "visitor", derive(Visit, VisitMut))]
        #[allow(non_camel_case_types)]
        pub enum Keyword {
            NoKeyword,
            $($ident),*
        }

        pub const ALL_KEYWORDS_INDEX: &[Keyword] = &[
            $(Keyword::$ident),*
        ];

        $(kw_def!($ident $(= $string_keyword)?);)*
        pub const ALL_KEYWORDS: &[&str] = &[
            $($ident),*
        ];
    };
}

// The following keywords should be sorted to be able to match using binary search
define_keywords!(
    ABORT,
    ABS,
    ABSOLUTE,
    ACTION,
    ADD,
    ADMIN,
    AGAINST,
    ALL,
    ALLOCATE,
    ALTER,
    ALWAYS,
    ANALYZE,
    AND,
    ANTI,
    ANY,
    APPLY,
    ARCHIVE,
    ARE,
    ARRAY,
    ARRAY_AGG,
    ARRAY_MAX_CARDINALITY,
    AS,
    ASC,
    ASENSITIVE,
    ASSERT,
    ASYMMETRIC,
    AT,
    ATOMIC,
    ATTACH,
    AUTHORIZATION,
    AUTO,
    AUTOINCREMENT,
    AUTO_INCREMENT,
    AVG,
    AVRO,
    BACKWARD,
    BASE64,
    BEGIN,
    BEGIN_FRAME,
    BEGIN_PARTITION,
    BETWEEN,
    BIGDECIMAL,
    BIGINT,
    BIGNUMERIC,
    BINARY,
    BINDING,
    BLOB,
    BLOOMFILTER,
    BOOL,
    BOOLEAN,
    BOTH,
    BROWSE,
    BTREE,
    BY,
    BYPASSRLS,
    BYTEA,
    BYTES,
    CACHE,
    CALL,
    CALLED,
    CARDINALITY,
    CASCADE,
    CASCADED,
    CASE,
    CAST,
    CEIL,
    CEILING,
    CENTURY,
    CHAIN,
    CHANGE,
    CHAR,
    CHARACTER,
    CHARACTERS,
    CHARACTER_LENGTH,
    CHARSET,
    CHAR_LENGTH,
    CHECK,
    CLOB,
    CLONE,
    CLOSE,
    CLUSTER,
    COALESCE,
    COLLATE,
    COLLATION,
    COLLECT,
    COLUMN,
    COLUMNS,
    COMMENT,
    COMMIT,
    COMMITTED,
    COMPRESSION,
    COMPUTE,
    CONCURRENTLY,
    CONDITION,
    CONFLICT,
    CONNECT,
    CONNECTION,
    CONSTRAINT,
    CONTAINS,
    CONVERT,
    COPY,
    COPY_OPTIONS,
    CORR,
    CORRESPONDING,
    COUNT,
    COVAR_POP,
    COVAR_SAMP,
    CREATE,
    CREATEDB,
    CREATEROLE,
    CREDENTIALS,
    CROSS,
    CSV,
    CUBE,
    CUME_DIST,
    CURRENT,
    CURRENT_CATALOG,
    CURRENT_DATE,
    CURRENT_DEFAULT_TRANSFORM_GROUP,
    CURRENT_PATH,
    CURRENT_ROLE,
    CURRENT_ROW,
    CURRENT_SCHEMA,
    CURRENT_TIME,
    CURRENT_TIMESTAMP,
    CURRENT_TRANSFORM_GROUP_FOR_TYPE,
    CURRENT_USER,
    CURSOR,
    CYCLE,
    DATA,
    DATABASE,
    DATE,
    DATETIME,
    DAY,
    DAYOFWEEK,
    DAYOFYEAR,
    DEALLOCATE,
    DEC,
    DECADE,
    DECIMAL,
    DECLARE,
    DEFAULT,
    DEFERRED,
    DELETE,
    DELIMITED,
    DELIMITER,
    DELTA,
    DENSE_RANK,
    DEREF,
    DESC,
    DESCRIBE,
    DETAIL,
    DETERMINISTIC,
    DIRECTORY,
    DISCARD,
    DISCONNECT,
    DISTINCT,
    DISTRIBUTE,
    DIV,
    DO,
    DOUBLE,
    DOW,
    DOY,
    DROP,
    DRY,
    DUPLICATE,
    DYNAMIC,
    EACH,
    ELEMENT,
    ELEMENTS,
    ELSE,
    EMPTY,
    ENCODING,
    ENCRYPTION,
    END,
    END_EXEC = "END-EXEC",
    ENDPOINT,
    END_FRAME,
    END_PARTITION,
    ENGINE,
    ENUM,
    EPOCH,
    EQUALS,
    ERROR,
    ESCAPE,
    EVENT,
    EVERY,
    EXCEPT,
    EXCLUDE,
    EXCLUSIVE,
    EXEC,
    EXECUTE,
    EXISTS,
    EXP,
    EXPANSION,
    EXPLAIN,
    EXPLICIT,
    EXTENDED,
    EXTERNAL,
    EXTRACT,
    FAIL,
    FALSE,
    FETCH,
    FIELDS,
    FILE,
    FILES,
    FILE_FORMAT,
    FILTER,
    FIRST,
    FIRST_VALUE,
    FLOAT,
    FLOAT4,
    FLOAT64,
    FLOAT8,
    FLOOR,
    FOLLOWING,
    FOR,
    FORCE,
    FORCE_NOT_NULL,
    FORCE_NULL,
    FORCE_QUOTE,
    FOREIGN,
    FORMAT,
    FORWARD,
    FRAME_ROW,
    FREE,
    FREEZE,
    FROM,
    FSCK,
    FULL,
    FULLTEXT,
    FUNCTION,
    FUNCTIONS,
    FUSION,
    GENERATE,
    GENERATED,
    GEOGRAPHY,
    GET,
    GLOBAL,
    GRANT,
    GRANTED,
    GRAPHVIZ,
    GROUP,
    GROUPING,
    GROUPS,
    HASH,
    HAVING,
    HEADER,
    HISTORY,
    HIVEVAR,
    HOLD,
    HOUR,
    HOURS,
    IDENTITY,
    IF,
    IGNORE,
    ILIKE,
    IMMEDIATE,
    IMMUTABLE,
    IN,
    INCLUDE,
    INCLUDE_NULL_VALUES,
    INCREMENT,
    INDEX,
    INDICATOR,
    INHERIT,
    INNER,
    INOUT,
    INPUTFORMAT,
    INSENSITIVE,
    INSERT,
    INT,
    INT2,
    INT4,
    INT64,
    INT8,
    INTEGER,
    INTERSECT,
    INTERSECTION,
    INTERVAL,
    INTO,
    IS,
    ISODOW,
    ISOLATION,
    ISOWEEK,
    ISOYEAR,
    JAR,
    JOIN,
    JSON,
    JSONFILE,
    JSON_TABLE,
    JULIAN,
    KEY,
    KILL,
    LAG,
    LANGUAGE,
    LARGE,
    LAST,
    LAST_VALUE,
    LATERAL,
    LEAD,
    LEADING,
    LEFT,
    LEVEL,
    LIKE,
    LIKE_REGEX,
    LIMIT,
    LISTAGG,
    LN,
    LOCAL,
    LOCALTIME,
    LOCALTIMESTAMP,
    LOCATION,
    LOCK,
    LOCKED,
    LOGIN,
    LOWER,
    LOW_PRIORITY,
    MACRO,
    MANAGEDLOCATION,
    MATCH,
    MATCHED,
    MATERIALIZED,
    MAX,
    MAXVALUE,
    MEDIUMINT,
    MEMBER,
    MERGE,
    METADATA,
    METHOD,
    MICROSECOND,
    MICROSECONDS,
    MILLENIUM,
    MILLENNIUM,
    MILLISECOND,
    MILLISECONDS,
    MIN,
    MINUTE,
    MINVALUE,
    MOD,
    MODE,
    MODIFIES,
    MODULE,
    MONTH,
    MSCK,
    MULTISET,
    MUTATION,
    NAME,
    NANOSECOND,
    NANOSECONDS,
    NATIONAL,
    NATURAL,
    NCHAR,
    NCLOB,
    NEW,
    NEXT,
    NO,
    NOBYPASSRLS,
    NOCREATEDB,
    NOCREATEROLE,
    NOINHERIT,
    NOLOGIN,
    NONE,
    NOREPLICATION,
    NORMALIZE,
    NOSCAN,
    NOSUPERUSER,
    NOT,
    NOTHING,
    NOWAIT,
    NTH_VALUE,
    NTILE,
    NULL,
    NULLIF,
    NULLS,
    NUMERIC,
    NVARCHAR,
    OBJECT,
    OCCURRENCES_REGEX,
    OCTETS,
    OCTET_LENGTH,
    OF,
    OFFSET,
    OLD,
    ON,
    ONLY,
    OPEN,
    OPERATOR,
    OPTIMIZE,
    OPTION,
    OPTIONS,
    OR,
    ORC,
    ORDER,
    OUT,
    OUTER,
    OUTPUTFORMAT,
    OVER,
    OVERFLOW,
    OVERLAPS,
    OVERLAY,
    OVERWRITE,
    OWNED,
    PARAMETER,
    PARQUET,
    PARTITION,
    PARTITIONED,
    PARTITIONS,
    PASSWORD,
    PATH,
    PATTERN,
    PERCENT,
    PERCENTILE_CONT,
    PERCENTILE_DISC,
    PERCENT_RANK,
    PERIOD,
    PIVOT,
    PLACING,
    PLANS,
    PORTION,
    POSITION,
    POSITION_REGEX,
    POWER,
    PRAGMA,
    PRECEDES,
    PRECEDING,
    PRECISION,
    PREPARE,
    PRESERVE,
    PRIMARY,
    PRIOR,
    PRIVILEGES,
    PROCEDURE,
    PROGRAM,
    PURGE,
    QUALIFY,
    QUARTER,
    QUERY,
    QUOTE,
    RANGE,
    RANK,
    RAW,
    RCFILE,
    READ,
    READS,
    REAL,
    RECURSIVE,
    REF,
    REFERENCES,
    REFERENCING,
    REGCLASS,
    REGEXP,
    REGR_AVGX,
    REGR_AVGY,
    REGR_COUNT,
    REGR_INTERCEPT,
    REGR_R2,
    REGR_SLOPE,
    REGR_SXX,
    REGR_SXY,
    REGR_SYY,
    RELATIVE,
    RELEASE,
    RENAME,
    REORG,
    REPAIR,
    REPEATABLE,
    REPLACE,
    REPLICATION,
    RESET,
    RESPECT,
    RESTRICT,
    RESULT,
    RETAIN,
    RETURN,
    RETURNING,
    RETURNS,
    REVOKE,
    RIGHT,
    RLIKE,
    ROLE,
    ROLLBACK,
    ROLLUP,
    ROOT,
    ROW,
    ROWID,
    ROWS,
    ROW_NUMBER,
    RUN,
    SAFE_CAST,
    SAVEPOINT,
    SCHEMA,
    SCOPE,
    SCROLL,
    SEARCH,
    SECOND,
    SELECT,
    SEMI,
    SENSITIVE,
    SEQUENCE,
    SEQUENCEFILE,
    SEQUENCES,
    SERDE,
    SERIALIZABLE,
    SESSION,
    SESSION_USER,
    SET,
    SETS,
    SHARE,
    SHOW,
    SIMILAR,
    SKIP,
    SMALLINT,
    SNAPSHOT,
    SOME,
    SORT,
    SPATIAL,
    SPECIFIC,
    SPECIFICTYPE,
    SQL,
    SQLEXCEPTION,
    SQLSTATE,
    SQLWARNING,
    SQRT,
    STABLE,
    STAGE,
    START,
    STATIC,
    STATISTICS,
    STDDEV_POP,
    STDDEV_SAMP,
    STDIN,
    STDOUT,
    STORAGE_INTEGRATION,
    STORED,
    STRICT,
    STRING,
    STRUCT,
    SUBMULTISET,
    SUBSTRING,
    SUBSTRING_REGEX,
    SUCCEEDS,
    SUM,
    SUPER,
    SUPERUSER,
    SWAP,
    SYMMETRIC,
    SYNC,
    SYSTEM,
    SYSTEM_TIME,
    SYSTEM_USER,
    TABLE,
    TABLES,
    TABLESAMPLE,
    TBLPROPERTIES,
    TEMP,
    TEMPORARY,
    TEXT,
    TEXTFILE,
    THEN,
    TIES,
    TIME,
    TIMESTAMP,
    TIMESTAMPTZ,
    TIMETZ,
    TIMEZONE,
    TIMEZONE_HOUR,
    TIMEZONE_MINUTE,
    TINYINT,
    TO,
    TOP,
    TRAILING,
    TRANSACTION,
    TRANSIENT,
    TRANSLATE,
    TRANSLATE_REGEX,
    TRANSLATION,
    TREAT,
    TRIGGER,
    TRIM,
    TRIM_ARRAY,
    TRUE,
    TRUNCATE,
    TRY_CAST,
    TYPE,
    UESCAPE,
    UNBOUNDED,
    UNCACHE,
    UNCOMMITTED,
    UNION,
    UNIQUE,
    UNKNOWN,
    UNLOCK,
    UNLOGGED,
    UNNEST,
    UNPIVOT,
    UNSIGNED,
    UNTIL,
    UPDATE,
    UPPER,
    URL,
    USAGE,
    USE,
    USER,
    USING,
    UUID,
    VACUUM,
    VALID,
    VALIDATION_MODE,
    VALUE,
    VALUES,
    VALUE_OF,
    VARBINARY,
    VARCHAR,
    VARIABLES,
    VARYING,
    VAR_POP,
    VAR_SAMP,
    VERBOSE,
    VERSIONING,
    VIEW,
    VIRTUAL,
    VOLATILE,
    WEEK,
    WHEN,
    WHENEVER,
    WHERE,
    WIDTH_BUCKET,
    WINDOW,
    WITH,
    WITHIN,
    WITHOUT,
    WITHOUT_ARRAY_WRAPPER,
    WORK,
    WRITE,
    XML,
    XOR,
    YEAR,
    ZONE,
    ZORDER
);

/// These keywords can't be used as a table alias, so that `FROM table_name alias`
/// can be parsed unambiguously without looking ahead.
pub const RESERVED_FOR_TABLE_ALIAS: &[Keyword] = &[
    // Reserved as both a table and a column alias:
    Keyword::WITH,
    Keyword::EXPLAIN,
    Keyword::ANALYZE,
    Keyword::SELECT,
    Keyword::WHERE,
    Keyword::GROUP,
    Keyword::SORT,
    Keyword::HAVING,
    Keyword::ORDER,
    Keyword::PIVOT,
    Keyword::UNPIVOT,
    Keyword::TOP,
    Keyword::LATERAL,
    Keyword::VIEW,
    Keyword::LIMIT,
    Keyword::OFFSET,
    Keyword::FETCH,
    Keyword::UNION,
    Keyword::EXCEPT,
    Keyword::INTERSECT,
    // Reserved only as a table alias in the `FROM`/`JOIN` clauses:
    Keyword::ON,
    Keyword::JOIN,
    Keyword::INNER,
    Keyword::CROSS,
    Keyword::FULL,
    Keyword::LEFT,
    Keyword::RIGHT,
    Keyword::NATURAL,
    Keyword::USING,
    Keyword::CLUSTER,
    Keyword::DISTRIBUTE,
    // for MSSQL-specific OUTER APPLY (seems reserved in most dialects)
    Keyword::OUTER,
    Keyword::SET,
    Keyword::QUALIFY,
    Keyword::WINDOW,
    Keyword::END,
    Keyword::FOR,
    // for MYSQL PARTITION SELECTION
    Keyword::PARTITION,
];

/// Can't be used as a column alias, so that `SELECT <expr> alias`
/// can be parsed unambiguously without looking ahead.
pub const RESERVED_FOR_COLUMN_ALIAS: &[Keyword] = &[
    // Reserved as both a table and a column alias:
    Keyword::WITH,
    Keyword::EXPLAIN,
    Keyword::ANALYZE,
    Keyword::SELECT,
    Keyword::WHERE,
    Keyword::GROUP,
    Keyword::SORT,
    Keyword::HAVING,
    Keyword::ORDER,
    Keyword::TOP,
    Keyword::LATERAL,
    Keyword::VIEW,
    Keyword::LIMIT,
    Keyword::OFFSET,
    Keyword::FETCH,
    Keyword::UNION,
    Keyword::EXCEPT,
    Keyword::INTERSECT,
    Keyword::CLUSTER,
    Keyword::DISTRIBUTE,
    // Reserved only as a column alias in the `SELECT` clause
    Keyword::FROM,
    Keyword::INTO,
    Keyword::END,
];
