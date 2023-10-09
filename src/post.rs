use std::collections::HashMap;

use patternfly_yew::prelude::*;
use plotters::{prelude::*, style::Color};
use plotters_canvas::CanvasBackend;
use web_sys::HtmlCanvasElement;
use yew::prelude::*;

use crate::model::{PostWithScores, Score, POSTS_WITH_SCORES};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Columns {
  Owner,
  Literary,
  Thinking,
  Mozheng,
  Sum,
}

impl TableEntryRenderer<Columns> for Score {
  fn render_cell(&self, ctx: CellContext<Columns>) -> Cell {
    match ctx.column {
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
        </Tooltip>),
    }
    .into()
  }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ScorePlotProps {
  pub scores: Vec<f64>,
  pub max_score: f64,
}

#[function_component(ScorePlot)]
pub fn score_plot(ScorePlotProps { scores, max_score }: &ScorePlotProps) -> Html {
  let canvas = use_node_ref();

  let data = use_memo(
    (scores.clone(), max_score.clone()),
    |(scores, max_score)| {
      let mut map = HashMap::new();
      for &s in scores {
        let key = (s / max_score * 5.).round() as i32;
        map.insert(key, map.get(&key).unwrap_or(&0) + 1);
      }
      map
    },
  );

  {
    let canvas = canvas.clone();
    let data = data.clone();

    use_effect_with(
      (canvas, data, max_score.clone()),
      |(canvas, data, max_score)| {
        let element = canvas.cast::<HtmlCanvasElement>().unwrap();

        element.set_height(400);
        element.set_width(1000);

        let root = CanvasBackend::with_canvas_object(element)
          .unwrap()
          .into_drawing_area();

        root.fill(&WHITE).unwrap();

        let y_max = *data.iter().map(|(_, v)| v).max().unwrap();

        let mut chart = ChartBuilder::on(&root)
          .x_label_area_size(35)
          .y_label_area_size(40)
          .margin(5)
          .build_cartesian_2d((-5..5).into_segmented(), 0..y_max)
          .unwrap();

        chart
          .configure_mesh()
          .disable_x_mesh()
          .bold_line_style(&WHITE.mix(0.3))
          .x_label_formatter(&|x| {
            format!(
              "{:.2}",
              *match x {
                SegmentValue::Exact(x) => x,
                SegmentValue::CenterOf(x) => x,
                SegmentValue::Last => unreachable!(),
              } as f64
                / 5.
                * max_score
            )
          })
          .y_desc("Count")
          .x_desc("Bucket")
          .axis_desc_style(("sans-serif", 15))
          .draw()
          .unwrap();

        chart
          .draw_series(
            Histogram::vertical(&chart)
              .style(BLACK.mix(0.5).filled())
              .data(data.iter().map(|(x, y)| (*x, *y))),
          )
          .unwrap();

        root.present().unwrap();
      },
    );
  }

  html! {
    <canvas ref={canvas} style="width: 100%" />
  }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PostProps {
  pub post_id: usize,
}

#[function_component(Post)]
pub fn post(PostProps { post_id }: &PostProps) -> Html {
  let PostWithScores {
    id: _,
    post,
    scores,
  } = POSTS_WITH_SCORES[*post_id].clone();

  let entries = use_state_eq(|| {
    let mut p = scores.clone();
    p.sort_by(|a, b| a.owner.cmp(&b.owner));
    p
  });

  let on_sort_by = {
    let entries = entries.clone();

    Some(Callback::from(move |val: TableHeaderSortBy<Columns>| {
      let mut entries_sorted = (*entries).clone();

      match val.index {
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
      <TableColumn<Columns> label="Owner" index={Columns::Owner} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Lit." index={Columns::Literary} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Thi." index={Columns::Thinking} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Moz." index={Columns::Mozheng} onsort={on_sort_by.clone()} />
      <TableColumn<Columns> label="Σ" index={Columns::Sum} onsort={on_sort_by.clone()} />
    </TableHeader<Columns>>
  };

  let s_literary: Vec<_> = scores.iter().map(|s| s.literary).collect();
  let s_thinking: Vec<_> = scores.iter().map(|s| s.thinking).collect();
  let s_mozheng: Vec<_> = scores.iter().map(|s| s.mozheng).collect();
  let s_sum: Vec<_> = scores.iter().map(|s| s.sum()).collect();

  let selected = use_state_eq(|| 0);
  let onselect = use_callback(selected.clone(), |index, selected| selected.set(index));

  html! {
    <>
      <Title level={Level::H1}>
        <a href={post.url.clone()}>{&post.title}</a>
        <sub>{format!("by {}", post.author)}</sub>
      </Title>
      <Tabs<usize> selected={*selected} {onselect}>
        <Tab<usize> index=0 title="Detail">
          <Table<Columns, UseTableData<Columns, UseStateTableModel<Score>>>
            {header}
            {entries}
          />
        </Tab<usize>>
        <Tab<usize> index=1 title="Literary">
          <ScorePlot scores={s_literary} max_score={1.0}/>
        </Tab<usize>>
        <Tab<usize> index=2 title="Thinking">
          <ScorePlot scores={s_thinking} max_score={1.0}/>
        </Tab<usize>>
        <Tab<usize> index=3 title="Mozheng">
          <ScorePlot scores={s_mozheng} max_score={1.0}/>
        </Tab<usize>>
        <Tab<usize> index=4 title="Sum">
          <ScorePlot scores={s_sum} max_score={3.5}/>
        </Tab<usize>>
      </Tabs<usize>>
    </>
  }
}
