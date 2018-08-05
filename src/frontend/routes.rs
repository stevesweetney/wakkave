//! All available routes within this application

// Credit: @saschagrunert

use frontend::services::router::Route;
use std::convert::Into;

#[derive(Debug, PartialEq)]
/// Possible child components of this one
pub enum RouterComponent {
    Feed,
    Loading,
    Login,
    Error,
}

/// Convert a RouterComponent into a Route
impl<T> Into<Route<T>> for RouterComponent
where
    T: Default,
{
    fn into(self) -> Route<T> {
        Route {
            fragment: Some(
                match self {
                    RouterComponent::Feed => "feed",
                    RouterComponent::Error => "error",
                    RouterComponent::Loading => "loading",
                    RouterComponent::Login => "login",
                }.into(),
            ),
            ..Default::default()
        }
    }
}

/// Convert a Route into a RouterComponent
impl<T> Into<RouterComponent> for Route<T> {
    fn into(self) -> RouterComponent {
        match self.fragment {
            Some(f) => match f.as_str() {
                "feed" => RouterComponent::Feed,
                "loading" => RouterComponent::Loading,
                "login" => RouterComponent::Login,
                _ => RouterComponent::Error,
            },
            _ => RouterComponent::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeed_to_convert_feed() {
        let sut: Route<()> = Route {
            fragment: Some("feed".to_owned()),
            ..Default::default()
        };
        let cmp: RouterComponent = sut.clone().into();
        assert_eq!(cmp, RouterComponent::Feed);
        assert_eq!(sut, RouterComponent::Feed.into());
    }

    #[test]
    fn succeed_to_convert_loading() {
        let sut: Route<()> = Route {
            fragment: Some("loading".to_owned()),
            ..Default::default()
        };
        let cmp: RouterComponent = sut.clone().into();
        assert_eq!(cmp, RouterComponent::Loading);
        assert_eq!(sut, RouterComponent::Loading.into());
    }

    #[test]
    fn succeed_to_convert_login() {
        let sut: Route<()> = Route {
            fragment: Some("login".to_owned()),
            ..Default::default()
        };
        let cmp: RouterComponent = sut.clone().into();
        assert_eq!(cmp, RouterComponent::Login);
        assert_eq!(sut, RouterComponent::Login.into());
    }

    #[test]
    fn succeed_to_convert_error() {
        let sut: Route<()> = Route {
            fragment: Some("error".to_owned()),
            ..Default::default()
        };
        let cmp: RouterComponent = sut.clone().into();
        assert_eq!(cmp, RouterComponent::Error);
        assert_eq!(sut, RouterComponent::Error.into());
    }

    #[test]
    fn succeed_to_convert_unknown() {
        let sut: Route<()> = Route {
            fragment: Some("new_route".to_owned()),
            ..Default::default()
        };
        let cmp: RouterComponent = sut.into();
        assert_eq!(cmp, RouterComponent::Error);
    }
}
