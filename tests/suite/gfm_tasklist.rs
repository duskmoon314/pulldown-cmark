// This file is auto-generated by the build script
// Please, do not modify it manually

use super::test_markdown_html;

#[test]
fn gfm_tasklist_test_1() {
    let original = r##"- [ ] foo
- [x] bar
"##;
    let expected = r##"<ul>
<li><input disabled="" type="checkbox"/>
foo</li>
<li><input disabled="" type="checkbox" checked=""/>
bar</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}

#[test]
fn gfm_tasklist_test_2() {
    let original = r##"- [x] foo
  - [ ] bar
  - [x] baz
- [ ] bim
"##;
    let expected = r##"<ul>
<li><input disabled="" type="checkbox" checked=""/>
foo
<ul>
<li><input disabled="" type="checkbox"/>
bar</li>
<li><input disabled="" type="checkbox" checked=""/>
baz</li>
</ul>
</li>
<li><input disabled="" type="checkbox"/>
bim</li>
</ul>
"##;

    test_markdown_html(original, expected, false, false, false);
}
