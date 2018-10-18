const BASE_URL: &str = "https://pixe.la/v1/";

pub fn users() -> String {
    String::from(format!("{}users", BASE_URL))
}

pub fn user(username: &str) -> String {
    String::from(format!("{}users/{}", BASE_URL, username))
}

pub fn graphs(username: &str) -> String {
    let user_url = user(username);
    String::from(format!("{}/graphs", user_url))
}

pub fn graph(username: &str, graph_id: &str) -> String {
    let graphs_url = graphs(username);
    String::from(format!("{}/{}", graphs_url, graph_id))
}

pub fn graph_svg(username: &str, graph_id: &str, date: Option<&str>) -> String {
    let graphs_url = graphs(username);

    if let Some(v) = date {
        // ToDo : query param move to HttpClient.
        return String::from(format!("{}/{}?date={}", graphs_url, graph_id, v))    
    }

    String::from(format!("{}/{}", graphs_url, graph_id))
}

pub fn pixel(username: &str, graph_id: &str, date: &str) -> String {
    let graph_url = graph(username, graph_id);
    String::from(format!("{}/{}", graph_url, date))
}

pub fn increment(username: &str, graph_id: &str) -> String {
    let graph_url = graph(username, graph_id);
    String::from(format!("{}/{}", graph_url, "increment"))
}

pub fn decrement(username: &str, graph_id: &str) -> String {
    let graph_url = graph(username, graph_id);
    String::from(format!("{}/{}", graph_url, "decrement"))
}


#[cfg(test)]
mod endpoint_test {
    mod user {
        use super::super::*;

        #[test]
        fn users_test() {
            assert_eq!(users(), "https://pixe.la/v1/users");
        }

        #[test]
        fn user_test() {
            let username = "testuser";
            let expect_url = String::from(format!("https://pixe.la/v1/users/{}", username));
            assert_eq!(user(username), expect_url);
        }
    }

    mod graph {
        use super::super::*;

        #[test]
        fn graphs_test() {
            let username = "testuser";
            let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs", username));
            assert_eq!(graphs(username), expect_url);
        }

        #[test]
        fn graph_test() {
            let username = "testuser";
            let graph_id = "testid";
            let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs/{}", username, graph_id));
            assert_eq!(graph(username, graph_id), expect_url);
        }

        #[test]
        fn graph_svg_test() {
            let username = "testuser";
            let graph_id = "testid";

            {
                let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs/{}", username, graph_id));
                assert_eq!(graph_svg(username, graph_id, None), expect_url);
            }
            {
                let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs/{}?date={}", username, graph_id, "20181018"));
                assert_eq!(graph_svg(username, graph_id, Some("20181018")), expect_url);
            }
        }
    }

    mod pixel {
        use super::super::*;

        #[test]
        fn pixel_test() {
            let username = "testuser";
            let graph_id = "testid";
            let date = "20180101";
            let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs/{}/{}", username, graph_id, date));
            assert_eq!(pixel(username, graph_id, date), expect_url);
        }

        #[test]
        fn increment_test() {
            let username = "testuser";
            let graph_id = "testid";
            let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs/{}/increment", username, graph_id));
            assert_eq!(increment(username, graph_id), expect_url);
        }

        #[test]
        fn decrement_test() {
            let username = "testuser";
            let graph_id = "testid";
            let expect_url = String::from(format!("https://pixe.la/v1/users/{}/graphs/{}/decrement", username, graph_id));
            assert_eq!(decrement(username, graph_id), expect_url);
        }
    }
}