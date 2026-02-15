use std::collections::BTreeMap;
use std::io::Result;
use std::io::Write;

const HELVETICA: &str = "Helvetica";
const HELVETICA_BOLD: &str = "Helvetica-Bold";
const HELVETICA_ITALICS: &str = "Helvetica-Oblique";
const HELVETICA_BOLD_ITALICS: &str = "Helvetica-BoldOblique";

pub fn init_page(content: &mut ContentBuilder) {
    content.save_graphics_state();
    content.set_colour(0., 0., 0.);
    content.set_colour_non_stroking(0., 0., 0.);
    content.stroke_path();
    content.save_graphics_state();
}

pub struct ContentBuilder<'a> {
    content: &'a mut Vec<Op>,
    page_size: (f64, f64),
}

#[derive(Clone, Copy)]
pub enum FontStyle {
    Normal,
    Bold,
    Italics,
    BoldItalics,
}

impl FontStyle {
    pub fn get_font_name(self) -> &'static str {
        match self {
            FontStyle::Normal => HELVETICA,
            FontStyle::Bold => HELVETICA_BOLD,
            FontStyle::Italics => HELVETICA_ITALICS,
            FontStyle::BoldItalics => HELVETICA_BOLD_ITALICS,
        }
    }
}

impl<'a> ContentBuilder<'a> {
    pub fn new(page_size: (f64, f64), content: &'a mut Vec<Op>) -> Self {
        Self { content, page_size }
    }

    pub fn page_size(&self) -> Coord {
        self.page_size.to_mm()
    }

    pub fn start_text_block(&mut self) {
        self.content.push(Op::BT);
    }

    pub fn fill(&mut self) {
        self.content.push(Op::f);
    }

    pub fn set_font(&mut self, style: FontStyle, size: f64) {
        let op = Op::Tf {
            font: style.get_font_name().to_owned(),
            size,
        };
        self.content.push(op);
    }

    pub fn save_graphics_state(&mut self) {
        self.content.push(Op::q);
    }

    pub fn restore_graphics_state(&mut self) {
        self.content.push(Op::Q);
    }

    pub fn stroke_path(&mut self) {
        self.content.push(Op::S);
    }
    pub fn set_leading(&mut self, leading: f64) {
        self.content.push(Op::TL(leading.to_inch().trim_fraction()));
    }

    pub fn set_colour(&mut self, r: f64, g: f64, b: f64) {
        self.content.push(Op::RG(r, g, b));
    }

    pub fn set_colour_non_stroking(&mut self, r: f64, g: f64, b: f64) {
        self.content.push(Op::rg(r, g, b));
    }

    pub fn set_text_position(&mut self, point: Coord) {
        let point = flip_y(point.to_inch(), self.page_size).trim_fraction();
        self.content.push(Op::Td(point));
    }

    pub fn print(&mut self, text: String) {
        self.content.push(Op::Tj(text));
    }

    pub fn print_at(&mut self, text: &str, position: Coord) {
        self.set_text_position(position);
        self.print(text.to_owned());
    }

    pub fn next_line(&mut self) {
        self.content.push(Op::TStar);
    }

    pub fn end_text_block(&mut self) {
        self.content.push(Op::ET);
    }

    pub fn line(&mut self, to: Coord) {
        self.content
            .push(Op::l(flip_y(to.to_inch(), self.page_size).trim_fraction()));
    }

    pub fn rectangle(&mut self, point: Coord, width: f64, height: f64) {
        let point = flip_y(point.to_inch(), self.page_size).trim_fraction();
        let height = -height.to_inch().trim_fraction();
        let width = width.to_inch().trim_fraction();
        self.content.push(Op::re {
            point,
            width,
            height,
        });
    }

    pub fn close_path(&mut self) {
        self.content.push(Op::h);
    }

    pub fn line_width(&mut self, width: f64) {
        self.content.push(Op::w(width.to_inch().trim_fraction()));
    }

    pub fn begin_subpath(&mut self, point: Coord) {
        let point = flip_y(point.to_inch(), self.page_size).trim_fraction();
        self.content.push(Op::m(point));
    }

    pub fn curve_to(&mut self, ctrl1: Coord, ctrl2: Coord, end: Coord) {
        let ctrl1 = flip_y(ctrl1.to_inch(), self.page_size).trim_fraction();
        let ctrl2 = flip_y(ctrl2.to_inch(), self.page_size).trim_fraction();
        let end = flip_y(end.to_inch(), self.page_size).trim_fraction();
        self.content.push(Op::c { ctrl1, ctrl2, end });
    }
}

fn flip_y((x, y): Coord, (_, y_page): Coord) -> Coord {
    (x, y_page - y)
}

pub const A5: Coord = (148.5 * 72. / 25.4, 210. * 72. / 25.4);

pub trait To72inch {
    fn to_inch(self) -> Self;
}
pub trait ToMM {
    fn to_mm(self) -> Self;
}

impl To72inch for f64 {
    fn to_inch(self) -> Self {
        self * 72. / 25.4
    }
}

impl ToMM for f64 {
    fn to_mm(self) -> Self {
        (self * 25.4 / 72.).trim_fraction()
    }
}

impl To72inch for Coord {
    fn to_inch(self) -> Self {
        let (x, y) = self;
        (x.to_inch(), y.to_inch())
    }
}

impl ToMM for Coord {
    fn to_mm(self) -> Self {
        let (x, y) = self;
        (x.to_mm(), y.to_mm())
    }
}

trait TrimFraction {
    fn trim_fraction(&self) -> Self;
}

impl TrimFraction for f64 {
    fn trim_fraction(&self) -> Self {
        (self * 10000.0).round() / 10000.0
    }
}

impl TrimFraction for (f64, f64) {
    fn trim_fraction(&self) -> Self {
        let (x, y) = self;
        (x.trim_fraction(), y.trim_fraction())
    }
}

pub enum PDFType {
    NumUsize(usize),
    NumF64(f64),
    Bool(bool),
    Name(NameObject),
    PDFString(String),
    Dictionary(DictionaryObject),
    IndirectRef(IndirectRef),
    Array(ArrayObject),
    PDFOp(Op),
    ContentStream(ContentStream),
    Null,
}

impl PDFWritable for PDFType {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let pdf_writer: &dyn PDFWritable = match self {
            PDFType::Bool(value) => value,
            PDFType::Dictionary(value) => value,
            PDFType::Name(value) => value,
            PDFType::NumF64(value) => value,
            PDFType::NumUsize(value) => value,
            PDFType::PDFString(value) => value,
            PDFType::IndirectRef(value) => value,
            PDFType::Array(value) => value,
            PDFType::PDFOp(value) => value,
            PDFType::ContentStream(value) => value,
            PDFType::Null => &PDFNull,
        };

        pdf_writer.write(writer)
    }
}

type Coord = (f64, f64);

#[allow(non_camel_case_types)]
pub enum Op {
    BT,
    ET,
    Td(Coord),
    TD(Coord),
    Tf {
        font: String,
        size: f64,
    },
    TL(f64),
    Tj(String),
    RG(f64, f64, f64),
    rg(f64, f64, f64),
    q,
    Q,
    S,
    h,
    f,
    re {
        point: Coord,
        width: f64,
        height: f64,
    },
    TStar,
    l(Coord),
    m(Coord),
    w(f64),
    c {
        ctrl1: Coord,
        ctrl2: Coord,
        end: Coord,
    },
}

impl ToPDFType for Op {
    fn to_pdftype(self) -> PDFType {
        PDFType::PDFOp(self)
    }
}

impl PDFWritable for Op {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut counting = CountingWriter::new(writer);
        match self {
            Op::BT => counting.write_str("BT"),
            Op::ET => counting.write_str("ET"),
            Op::q => counting.write_str("q"),
            Op::Q => counting.write_str("Q"),
            Op::S => counting.write_str("S"),
            Op::h => counting.write_str("h"),
            Op::f => counting.write_str("f"),
            Op::TStar => counting.write_str("T*"),
            Op::re {
                point: (x, y),
                width,
                height,
            } => counting.write_str(&format!("{x} {y} {width} {height} re")),
            Op::TL(leading) => counting.write_str(&format!("{leading} TL")),
            Op::Td((x, y)) => counting.write_str(&format!("{x} {y} Td")),
            Op::TD((x, y)) => counting.write_str(&format!("{x} {y} TD")),
            Op::RG(r, g, b) => counting.write_str(&format!("{r} {g} {b} RG")),
            Op::rg(r, g, b) => counting.write_str(&format!("{r} {g} {b} rg")),
            Op::l((x, y)) => counting.write_str(&format!("{x} {y} l")),
            Op::m((x, y)) => counting.write_str(&format!("{x} {y} m")),
            Op::w(width) => counting.write_str(&format!("{width} w")),
            Op::c {
                ctrl1: (x1, y1),
                ctrl2: (x2, y2),
                end: (x3, y3),
            } => counting.write_str(&format!("{x1} {y1} {x2} {y2} {x3} {y3} c")),
            Op::Tf { font, size } => {
                let name = NameObject::new(font);
                name.write(&mut counting)?;
                counting.write_str(&format!(" {size} Tf"))
            }
            Op::Tj(string) => {
                string.write(&mut counting)?;
                counting.write_str(" Tj")
            }
        }
    }
}

struct PDFAllocator {
    contents: Vec<PDFType>,
}

impl PDFAllocator {
    fn new() -> Self {
        let contents = vec![];
        Self { contents }
    }

    pub fn alloc<T: ToPDFType>(&mut self, obj: T) -> IndirectRef {
        let id = self.contents.len();
        self.contents.push(obj.to_pdftype());

        IndirectRef { id }
    }

    pub fn peek_alloc(&mut self) -> PeekAlloc {
        let id = self.contents.len();
        self.contents.push(PDFType::Null);
        let indirect = IndirectRef { id };

        PeekAlloc { indirect }
    }

    fn into_vec(self) -> Vec<PDFType> {
        self.contents
    }
}

struct PeekAlloc {
    indirect: IndirectRef,
}

impl PeekAlloc {
    fn complete<T: ToPDFType>(self, alloc: &mut PDFAllocator, obj: T) {
        alloc.contents[self.indirect.id] = obj.to_pdftype();
    }

    fn indirect(&self) -> IndirectRef {
        self.indirect
    }
}

pub struct PDFDocumentBuilder {
    alloc: PDFAllocator,
    pages: Vec<PageStructure>,
    page_resources: IndirectRef,
}

struct PageStructure {
    page_dict: DictionaryObject,
    contents: ContentStream,
}

impl PDFDocumentBuilder {
    pub fn new() -> Self {
        let mut alloc = PDFAllocator::new();

        fn create_font(name: &str, alloc: &mut PDFAllocator) -> IndirectRef {
            let mut dict = DictionaryObject::new();
            dict.insert_strkey("Type", NameObject::new("Font"));
            dict.insert_strkey("Subtype", NameObject::new("Type1"));
            dict.insert_strkey("BaseFont", NameObject::new(name));
            dict.insert_strkey("Encoding", NameObject::new("WinAnsiEncoding"));
            alloc.alloc(dict)
        }

        let font = create_font(HELVETICA, &mut alloc);
        let font_bold = create_font(HELVETICA_BOLD, &mut alloc);
        let font_italics = create_font(HELVETICA_ITALICS, &mut alloc);
        let font_bold_italics = create_font(HELVETICA_BOLD_ITALICS, &mut alloc);

        let page_resources = {
            let mut font_dict = DictionaryObject::new();
            font_dict.insert_strkey(HELVETICA, font);
            font_dict.insert_strkey(HELVETICA_BOLD, font_bold);
            font_dict.insert_strkey(HELVETICA_ITALICS, font_italics);
            font_dict.insert_strkey(HELVETICA_BOLD_ITALICS, font_bold_italics);
            let font_ref = alloc.alloc(font_dict);

            let mut resources = DictionaryObject::new();
            resources.insert_strkey("Font", font_ref);
            alloc.alloc(resources)
        };

        let pages = vec![];

        Self {
            alloc,
            pages,
            page_resources,
        }
    }

    pub fn create_page(&mut self, page_size: (f64, f64)) -> PDFPageBuilder<'_> {
        let page_dic_vec = DictionaryObject::new_page();
        let contents = ContentStream::new();

        let page_structure = PageStructure {
            page_dict: page_dic_vec,
            contents,
        };

        self.pages.push(page_structure);
        let page_structure = self.pages.last_mut().unwrap();

        let (page_x, page_y) = page_size.trim_fraction();
        let mediabox = ArrayObject::new_from([0., 0., page_x, page_y]);
        page_structure
            .page_dict
            .insert_strkey("Resources", DictionaryObject::new());
        page_structure.page_dict.insert_strkey("MediaBox", mediabox);
        page_structure
            .page_dict
            .insert_strkey("Resources", self.page_resources);

        PDFPageBuilder {
            page_structure,
            page_size,
        }
    }

    pub fn to_doc(mut self) -> PDFDocument {
        let mut catalog = DictionaryObject::new_document_catalog();

        let mut pages = DictionaryObject::new_pages();
        let pages_peek = self.alloc.peek_alloc();
        let catalog_peek = self.alloc.peek_alloc();

        {
            let mut page_array = ArrayObject::new();
            for mut page_structure in self.pages.into_iter() {
                let content_ref = self.alloc.alloc(page_structure.contents);
                page_structure
                    .page_dict
                    .insert_strkey("Parent", pages_peek.indirect());
                page_structure
                    .page_dict
                    .insert_strkey("Contents", content_ref);
                let indirect = self.alloc.alloc(page_structure.page_dict);
                page_array.push(indirect)
            }
            pages.insert_strkey("Count", page_array.len());
            pages.insert_strkey("Kids", page_array);
        };

        catalog.insert_strkey("Pages", pages_peek.indirect());
        catalog.insert_strkey("PageLayout", "OneColumn");

        let root = catalog_peek.indirect();

        pages_peek.complete(&mut self.alloc, pages);
        catalog_peek.complete(&mut self.alloc, catalog);

        let content = vec![];
        let indirect = self.alloc.into_vec();

        PDFDocument {
            content,
            indirect,
            root,
        }
    }
}

pub struct PDFPageBuilder<'a> {
    page_structure: &'a mut PageStructure,
    page_size: (f64, f64),
}

impl<'a> PDFPageBuilder<'a> {
    pub fn contents(&mut self) -> &mut Vec<Op> {
        self.page_structure.contents.contents()
    }

    pub fn content_builder(&mut self) -> ContentBuilder<'_> {
        ContentBuilder::new(self.page_size, self.page_structure.contents.contents())
    }
}

pub struct PDFDocument {
    content: Vec<PDFType>,
    indirect: Vec<PDFType>,
    root: IndirectRef,
}

impl PDFDocument {
    pub fn write<T: Write>(&self, writer: &mut T) -> Result<usize> {
        let mut counting = CountingWriter::new(writer);

        counting.write_str_ln("%PDF-1.5")?;
        for pdf_obj in self.content.iter() {
            pdf_obj.write(&mut counting)?;
            counting.write_str("\n")?;
        }

        let mut xref = vec![];
        for (id, v) in self.indirect.iter().enumerate() {
            let offset = counting.size();
            xref.push(offset);

            write_indirect(id, v, &mut counting)?;
            counting.write_str("\n")?;
        }

        let startxref = counting.size();
        let num_xref = xref.len() + 1;

        counting.write_str_ln("xref")?;
        counting.write_str_ln(format!("0 {}", num_xref).as_str())?;
        counting.write_str_ln("0000000000 65535 f ")?;
        for offset in xref {
            let line = format!("{:0>10} 00000 n ", offset);
            counting.write_str_ln(line.as_str())?;
        }

        let mut trailer = DictionaryObject::new();
        trailer.insert_strkey("Size", num_xref);

        trailer.insert_strkey("Root", self.root);

        counting.write_str_ln("trailer")?;
        trailer.write(&mut counting)?;

        counting.write_ln()?;

        counting.write_str_ln("startxref")?;
        counting.write_str_ln(format!("{startxref}").as_str())?;
        counting.write_str_ln("%%EOF")?;
        Ok(counting.size())
    }
}

impl<'a> Write for CountingWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

struct CountingWriter<'a> {
    writer: &'a mut dyn Write,
    size: usize,
}

impl<'a> CountingWriter<'a> {
    fn new(writer: &'a mut dyn Write) -> Self {
        let size = 0;

        Self { size, writer }
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let size = self.writer.write(buf)?;
        self.size += size;
        Ok(size)
    }

    fn write_str(&mut self, msg: &str) -> Result<usize> {
        let size = self.writer.write(msg.as_bytes())?;
        self.size += size;
        Ok(size)
    }

    fn write_str_ln(&mut self, msg: &str) -> Result<usize> {
        let mut size = self.writer.write(msg.as_bytes())?;
        size += self.writer.write("\n".as_bytes())?;
        self.size += size;
        Ok(size)
    }

    fn write_ln(&mut self) -> Result<usize> {
        let size = self.writer.write("\n".as_bytes())?;
        self.size += size;
        Ok(size)
    }

    fn size(&self) -> usize {
        self.size
    }
}

pub trait PDFWritable {
    fn write(&self, writer: &mut dyn Write) -> Result<usize>;
}

pub trait ToPDFType {
    fn to_pdftype(self) -> PDFType;
}

impl ToPDFType for String {
    fn to_pdftype(self) -> PDFType {
        PDFType::PDFString(self)
    }
}

impl ToPDFType for &str {
    fn to_pdftype(self) -> PDFType {
        PDFType::PDFString(self.to_owned())
    }
}

impl ToPDFType for NameObject {
    fn to_pdftype(self) -> PDFType {
        PDFType::Name(self)
    }
}

impl ToPDFType for f64 {
    fn to_pdftype(self) -> PDFType {
        PDFType::NumF64(self)
    }
}

impl ToPDFType for usize {
    fn to_pdftype(self) -> PDFType {
        PDFType::NumUsize(self)
    }
}

impl ToPDFType for bool {
    fn to_pdftype(self) -> PDFType {
        PDFType::Bool(self)
    }
}

impl ToPDFType for DictionaryObject {
    fn to_pdftype(self) -> PDFType {
        PDFType::Dictionary(self)
    }
}

impl ToPDFType for IndirectRef {
    fn to_pdftype(self) -> PDFType {
        PDFType::IndirectRef(self)
    }
}

impl ToPDFType for ArrayObject {
    fn to_pdftype(self) -> PDFType {
        PDFType::Array(self)
    }
}

impl PDFWritable for usize {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let value = format!("{}", self);
        writer.write(value.as_bytes())
    }
}

impl PDFWritable for f64 {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let value = format!("{}", self);
        writer.write(value.as_bytes())
    }
}

impl PDFWritable for bool {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let value = if *self { "true" } else { "false" };
        writer.write(value.as_bytes())
    }
}

#[derive(Ord, Eq, PartialEq, PartialOrd)]
pub struct NameObject {
    name: String,
}

impl NameObject {
    pub fn new(name: &str) -> Self {
        NameObject {
            name: name.to_owned(),
        }
    }
}

impl PDFWritable for NameObject {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut counting = CountingWriter::new(writer);

        counting.write_str(&format!("/{}", self.name))?;
        Ok(counting.size())
    }
}

impl PDFWritable for String {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut result: String = String::new();

        for ch in self.chars() {
            match ch {
                '\n' => add_all(&mut result, "\\n"),
                '\r' => add_all(&mut result, "\\r"),
                '\u{0C}' => add_all(&mut result, "\\f"),
                '\u{08}' => add_all(&mut result, "\\b"),
                '(' => add_all(&mut result, "\\("),
                ')' => add_all(&mut result, "\\)"),
                '\\' => add_all(&mut result, "\\\\"),
                c if (c as u32) > 0x7f => add_all(&mut result, &format!("\\{:03o}", c as u32)),
                c => result.push(c),
            }
        }

        writer.write(format!("({result})").as_bytes())
    }
}

fn add_all(vec: &mut String, values: &str) {
    for value in values.chars() {
        vec.push(value);
    }
}

pub struct PDFNull;

impl PDFWritable for PDFNull {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        writer.write("null".as_bytes())
    }
}

pub struct DictionaryObject {
    map: BTreeMap<NameObject, PDFType>,
}

impl DictionaryObject {
    pub fn new() -> Self {
        let map = BTreeMap::new();
        Self { map }
    }

    pub fn new_pages() -> Self {
        let mut dict = DictionaryObject::new();
        dict.insert_strkey("Type", NameObject::new("Pages"));
        dict
    }

    pub fn new_page() -> Self {
        let mut dict = DictionaryObject::new();
        dict.insert_strkey("Type", NameObject::new("Page"));
        dict
    }

    pub fn new_document_catalog() -> Self {
        let mut dict = DictionaryObject::new();
        dict.insert_strkey("Type", NameObject::new("Catalog"));
        dict
    }

    pub fn insert<T: ToPDFType>(&mut self, key: NameObject, value: T) {
        self.map.insert(key, value.to_pdftype());
    }

    pub fn insert_strkey<T: ToPDFType>(&mut self, key: &str, value: T) {
        self.map.insert(NameObject::new(key), value.to_pdftype());
    }

    pub fn get_mut(&mut self, key: String) -> Option<&mut PDFType> {
        let named_key = NameObject::new(&key);
        self.map.get_mut(&named_key)
    }
}

impl PDFWritable for DictionaryObject {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut counting = CountingWriter::new(writer);
        counting.write_str("<<")?;

        for (k, v) in self.map.iter() {
            k.write(&mut counting)?;
            counting.write_str(" ")?;
            v.write(&mut counting)?;
            counting.write_str("\n")?;
        }

        counting.write_str(">>")?;
        Ok(counting.size())
    }
}

pub struct ContentStream {
    contents: Vec<Op>,
}

impl ToPDFType for ContentStream {
    fn to_pdftype(self) -> PDFType {
        PDFType::ContentStream(self)
    }
}

impl PDFWritable for ContentStream {
    #[allow(clippy::unused_io_amount)]
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut byte_contents = vec![];

        for content in self.contents.iter() {
            content.write(&mut byte_contents)?;
            // Written size handled later
            byte_contents.write("\n".as_bytes())?;
        }

        let mut dict = DictionaryObject::new();
        dict.insert_strkey("Length", byte_contents.len());

        let mut counting = CountingWriter::new(writer);
        dict.write(&mut counting)?;
        counting.write_str_ln("\nstream")?;
        counting.write(&byte_contents)?;
        counting.write_str("endstream")
    }
}

impl ContentStream {
    pub fn new() -> Self {
        let contents = vec![];
        Self { contents }
    }

    pub fn contents(&mut self) -> &mut Vec<Op> {
        &mut self.contents
    }
}

pub struct IndirectObject {
    pdf_obj: Box<PDFType>,
    id: usize,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy)]
pub struct IndirectRef {
    id: usize,
}

impl PDFWritable for IndirectRef {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let line = format!("{} 0 R", self.id + 1);
        writer.write(line.as_bytes())
    }
}

impl IndirectObject {
    pub fn new(id: usize, pdf_obj: Box<PDFType>) -> Self {
        Self { id, pdf_obj }
    }

    pub fn pdf_ref(&self) -> IndirectRef {
        IndirectRef { id: self.id }
    }
}

fn write_indirect(id: usize, value: &PDFType, writer: &mut dyn Write) -> Result<usize> {
    let mut counting = CountingWriter::new(writer);
    let start = format!("{} 0 obj\n", id + 1);
    counting.write_str(start.as_str())?;

    value.write(&mut counting)?;
    counting.write_str("\nendobj")?;

    Ok(counting.size())
}

impl PDFWritable for IndirectObject {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        write_indirect(self.id, &self.pdf_obj, writer)
    }
}

pub struct ArrayObject {
    array: Vec<PDFType>,
}

impl ArrayObject {
    pub fn new() -> Self {
        let array = vec![];
        Self { array }
    }

    pub fn new_from<T: ToPDFType, const N: usize>(values: [T; N]) -> Self {
        let mut array = vec![];
        for value in values {
            array.push(value.to_pdftype());
        }
        Self { array }
    }

    pub fn push<T: ToPDFType>(&mut self, value: T) {
        self.array.push(value.to_pdftype());
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }
}

impl PDFWritable for ArrayObject {
    fn write(&self, writer: &mut dyn Write) -> Result<usize> {
        let mut counting = CountingWriter::new(writer);

        counting.write_str("[")?;
        for (idx, obj) in self.array.iter().enumerate() {
            if idx > 0 {
                counting.write_str(" ")?;
            }
            obj.write(&mut counting)?;
        }
        counting.write_str("]")?;

        Ok(counting.size())
    }
}

impl Default for DictionaryObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ArrayObject {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ContentStream {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PDFDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
