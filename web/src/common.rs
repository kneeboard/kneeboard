use crate::icons::{chevron_bar_down, chevron_bar_up, x_circle};

use web_sys::{Event, FileList, HtmlInputElement};

use yew::prelude::*;

pub fn to_number(e: Event) -> i64 {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.value_as_number() as i64
}

pub fn to_string(e: Event) -> String {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.value()
}

pub fn to_files(e: Event) -> Option<FileList> {
    let input: HtmlInputElement = e.target_unchecked_into();
    input.files()
}

pub fn append_insert_delete(
    append: Callback<MouseEvent>,
    insert: Callback<MouseEvent>,
    delete: Callback<MouseEvent>,
    is_last: bool,
) -> Html {
    html!(
       <table>
       <tr>
         <td style="padding: 0px" valign="top">
           <button type="button" class="btn btn-link btn-sm py-0 px-0" onclick={insert}>
             {chevron_bar_up(14)}
           </button>
         </td>
       </tr>
       <tr>
         <td style="padding: 0px" valign="middle">
         <button type="button" class="btn btn-link btn-sm py-0 px-0" onclick={delete}>
            {x_circle(14)}
         </button>
         </td>
       </tr>
       if is_last {
       <tr>
       <td style="padding: 0px" valign="bottom">
         <button type="button" class="btn btn-link btn-sm py-0 px-0" onclick={append}>
           {chevron_bar_down(14)}
         </button>
         </td>
       </tr>
       }
     </table>
    )
}

pub trait IsLast {
    fn is_last(&self, idx: usize) -> bool;
}

impl<T> IsLast for [T] {
    fn is_last(&self, idx: usize) -> bool {
        self.len() - 1 == idx
    }
}
