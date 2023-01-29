pub enum TokenKind {
    FileStart,
    FileEnd,
    Heading,

    BeginUnorderedList,
    UnorderedListItem,
    EndUnorderedList,

    BeginOrderedList,
    OrderedListItem,
    EndOrderedList,

    BeginCodeBlock,
    BodyCodeBlock,
    EndCodeBlock,

    BeginBlockQuote,
    BodyBlockQuote,
    EndBlockQuote,

    Text,

    Blank,
}
