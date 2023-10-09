use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::model::{Score, POSTS, SCORES};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Columns {
  PostTitle,
  Owner,
  Literary,
  Thinking,
  Mozheng,
  Sum,
}

fn limit_string(s: &str, limit: usize) -> String {
  if s.len() > limit {
    format!("{}…", String::from_utf8_lossy(&s.as_bytes()[..limit - 2]))
  } else {
    s.to_string()
  }
}

const TITLE_LIMIT: usize = 24;

impl TableEntryRenderer<Columns> for Score {
  fn render_cell(&self, ctx: CellContext<Columns>) -> Cell {
    match ctx.column {
      Columns::PostTitle => html!(
        <Tooltip text={POSTS[self.post_id].title.clone()}>
          { limit_string(&POSTS[self.post_id].title, TITLE_LIMIT) }
        </Tooltip>
      ),
      Columns::Owner => html! { &self.owner },
      Columns::Literary => html!(
        <Tooltip text={format!("Literary: {:.9}", self.literary)}>
          {format!("{:.3}", self.literary)}
        </Tooltip>
      ),
      Columns::Thinking => html!(
        <Tooltip text={format!("Thinking: {:.9}", self.thinking)}>
          {format!("{:.3}", self.thinking)}
        </Tooltip>
      ),
      Columns::Mozheng => html!(
        <Tooltip text={format!("Mozheng: {:.9}", self.mozheng)}>
          {format!("{:.3}", self.mozheng)}
        </Tooltip>
      ),
      Columns::Sum => html!(
        <Tooltip text={format!("Sum: {:.9}", self.sum())}>
          {format!("{:.3}", self.sum())}
        </Tooltip>
      ),
    }
    .into()
  }
}

#[function_component(Status)]
pub fn status() -> Html {
  let entries = use_state_eq(|| {
    let mut p = SCORES.clone();
    p.sort_by(|a, b| POSTS[a.post_id].title.cmp(&POSTS[b.post_id].title));
    p
  });

  let on_sort_by = {
    let entries = entries.clone();

    Some(Callback::from(move |val: TableHeaderSortBy<Columns>| {
      let mut entries_sorted = (*entries).clone();

      match val.index {
        Columns::PostTitle => {
          entries_sorted.sort_by(|a, b| POSTS[a.post_id].title.cmp(&POSTS[b.post_id].title));
        }
        Columns::Owner => {
          entries_sorted.sort_by(|a, b| a.owner.cmp(&b.owner));
        }
        Columns::Literary => {
          entries_sorted.sort_by(|a, b| a.literary.partial_cmp(&b.literary).unwrap());
        }
        Columns::Thinking => {
          entries_sorted.sort_by(|a, b| a.thinking.partial_cmp(&b.thinking).unwrap());
        }
        Columns::Mozheng => {
          entries_sorted.sort_by(|a, b| a.mozheng.partial_cmp(&b.mozheng).unwrap());
        }
        Columns::Sum => {
          entries_sorted.sort_by(|a, b| a.sum().partial_cmp(&b.sum()).unwrap());
        }
      };

      if !val.asc {
        entries_sorted.reverse();
      }
      entries.set(entries_sorted);
    }))
  };

  let (entries, _) = use_table_data(UseStateTableModel::new(entries));

  let header = html_nested! {
    <TableHeader<Columns>>
      <TableColumn<Columns> label="Post" index={Columns::PostTitle} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Owner" index={Columns::Owner} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Lit." index={Columns::Literary} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Thi." index={Columns::Thinking} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Moz." index={Columns::Mozheng} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Σ" index={Columns::Sum} onsort={on_sort_by.clone()} />
    </TableHeader<Columns>>
  };

  html! (
    <Table<Columns, UseTableData<Columns, UseStateTableModel<Score>>>
      {header}
      {entries}
    />
  )
}
