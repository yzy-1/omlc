use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::components::Link;

use crate::{
  app::AppRoute,
  model::{PostWithScores, POSTS_WITH_SCORES},
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Columns {
  Title,
  Author,
  LiteraryAvg,
  LiteraryVar,
  ThinkingAvg,
  ThinkingVar,
  MozhengAvg,
  MozhengVar,
  SumAvg,
  SumVar,
  Open,
}

fn limit_string(s: &str, limit: usize) -> String {
  if s.len() > limit {
    format!("{}...", String::from_utf8_lossy(&s.as_bytes()[..limit - 3]))
  } else {
    s.to_string()
  }
}

const TITLE_LIMIT: usize = 16;

impl TableEntryRenderer<Columns> for PostWithScores {
  fn render_cell(&self, ctx: CellContext<Columns>) -> Cell {
    match ctx.column {
      Columns::Title => html!(
        <Tooltip text={self.post.title.to_string()}>
          <a href={self.post.url.clone()}>{ limit_string(&self.post.title, TITLE_LIMIT) }</a>
        </Tooltip>
      ),
      Columns::Author => html! { &self.post.author },
      Columns::LiteraryAvg => html!(
        <Tooltip text={format!("Literary Average: {:.9}", self.literary_avg())}>
          {format!("{:.3}", self.literary_avg())}
        </Tooltip>
      ),
      Columns::LiteraryVar => html!(
        <Tooltip text={format!("Literary Variance: {:.9}", self.literary_var())}>
          {format!("{:.3}", self.literary_var())}
        </Tooltip>
      ),
      Columns::ThinkingAvg => html!(
        <Tooltip text={format!("Thinking Average: {:.9}", self.thinking_avg())}>
          {format!("{:.3}", self.thinking_avg())}
        </Tooltip>
      ),
      Columns::ThinkingVar => html!(
        <Tooltip text={format!("Thinking Variance: {:.9}", self.thinking_var())}>
          {format!("{:.3}", self.thinking_var())}
        </Tooltip>
      ),
      Columns::MozhengAvg => html!(
        <Tooltip text={format!("Mozheng Average: {:.9}", self.mozheng_avg())}>
          {format!("{:.3}", self.mozheng_avg())}
        </Tooltip>
      ),
      Columns::MozhengVar => html!(
        <Tooltip text={format!("Mozheng Variance: {:.9}", self.mozheng_var())}>
          {format!("{:.3}", self.mozheng_var())}
        </Tooltip>
      ),
      Columns::SumAvg => html!(
        <Tooltip text={format!("Sum Average: {:.9}", self.sum_avg())}>
          {format!("{:.3}", self.sum_avg())}
        </Tooltip>
      ),
      Columns::SumVar => html!(
        <Tooltip text={format!("Sum Variance: {:.9}", self.sum_var())}>
          {format!("{:.3}", self.sum_var())}
        </Tooltip>
      ),
      Columns::Open => html!(
        <Link<AppRoute> target={AppRoute::Post { id: self.id }}>{"Open"}</Link<AppRoute>>
      ),
    }
    .into()
  }
}

#[function_component(Board)]
pub fn board() -> Html {
  let entries = use_state_eq(|| {
    let mut p = POSTS_WITH_SCORES.clone();
    p.sort_by(|a, b| a.post.title.cmp(&b.post.title));
    p
  });

  let on_sort_by = {
    let entries = entries.clone();

    Some(Callback::from(move |val: TableHeaderSortBy<Columns>| {
      let mut entries_sorted = (*entries).clone();

      match val.index {
        Columns::Title => {
          entries_sorted.sort_by(|a, b| a.post.title.cmp(&b.post.title));
        }
        Columns::Author => {
          entries_sorted.sort_by(|a, b| a.post.author.cmp(&b.post.author));
        }
        Columns::LiteraryAvg => {
          entries_sorted.sort_by(|a, b| a.literary_avg().total_cmp(&b.literary_avg()));
        }
        Columns::LiteraryVar => {
          entries_sorted.sort_by(|a, b| a.literary_var().total_cmp(&b.literary_var()));
        }
        Columns::ThinkingAvg => {
          entries_sorted.sort_by(|a, b| a.thinking_avg().total_cmp(&b.thinking_avg()));
        }
        Columns::ThinkingVar => {
          entries_sorted.sort_by(|a, b| a.thinking_var().total_cmp(&b.thinking_var()));
        }
        Columns::MozhengAvg => {
          entries_sorted.sort_by(|a, b| a.mozheng_avg().total_cmp(&b.mozheng_avg()));
        }
        Columns::MozhengVar => {
          entries_sorted.sort_by(|a, b| a.mozheng_var().total_cmp(&b.mozheng_var()));
        }
        Columns::SumAvg => {
          entries_sorted.sort_by(|a, b| a.sum_avg().total_cmp(&b.sum_avg()));
        }
        Columns::SumVar => {
          entries_sorted.sort_by(|a, b| a.sum_var().total_cmp(&b.sum_var()));
        }
        _ => {}
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
      <TableColumn<Columns> label="Title" index={Columns::Title} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Author" index={Columns::Author} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Lit. Avg" index={Columns::LiteraryAvg} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Lit. Var" index={Columns::LiteraryVar} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Thi. Avg" index={Columns::ThinkingAvg} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Thi. Var" index={Columns::ThinkingVar} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Moz. Avg" index={Columns::MozhengAvg} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Moz. Var" index={Columns::MozhengVar} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Σ Avg" index={Columns::SumAvg} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Σ Var" index={Columns::SumVar} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="" index={Columns::Open} />
    </TableHeader<Columns>>
  };

  html! (
    <Table<Columns, UseTableData<Columns, UseStateTableModel<PostWithScores>>>
      {header}
      {entries}
    />
  )
}
