use patternfly_yew::prelude::*;
use yew::prelude::*;
use yew_nested_router::prelude::{Switch as RouterSwitch, *};

use crate::{board::Board, post::Post, status::Status};

#[derive(Debug, Clone, PartialEq, Eq, Target)]
pub enum AppRoute {
  #[target(index)]
  Home,
  #[target(rename = "b")]
  Board,
  #[target(rename = "s")]
  Status,
  #[target(rename = "p")]
  Post { id: usize },
}

pub fn switch_app_route(routes: AppRoute) -> Html {
  let inner = match routes {
    AppRoute::Home => html! { <h1>{ "Home" }</h1> },
    AppRoute::Board => html! { <Board /> },
    AppRoute::Status => html! { <Status /> },
    AppRoute::Post { id } => html! { <Post post_id={id} /> },
  };

  html! { <AppPage>{inner}</AppPage>}
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PageProps {
  pub children: Children,
}

#[function_component(AppPage)]
fn page(props: &PageProps) -> Html {
  let sidebar = html_nested! {
    <PageSidebar>
      <Nav>
        <NavList>
          <NavRouterItem<AppRoute> to={AppRoute::Home}>{"Home"}</NavRouterItem<AppRoute>>
          <NavRouterItem<AppRoute> to={AppRoute::Board}>{"Board"}</NavRouterItem<AppRoute>>
          <NavRouterItem<AppRoute> to={AppRoute::Status}>{"Status"}</NavRouterItem<AppRoute>>
        </NavList>
      </Nav>
    </PageSidebar>
  };

  let brand = html_nested! {"Open Mozheng Literature Cup"};

  html! {
    <Page {brand} {sidebar}>
      { for props.children.iter() }
    </Page>
  }
}

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <Router<AppRoute>>
      <RouterSwitch<AppRoute> render={switch_app_route} />
    </Router<AppRoute>>
  }
}
