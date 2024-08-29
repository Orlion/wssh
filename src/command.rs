use crate::{
    error,
    jumpserver::{AppItem, Jumpserver, Pod},
    terminal,
};

pub struct Command {
    clap_command: clap::Command,
    jumpserver: Jumpserver,
}

impl Command {
    pub fn new(command: clap::Command, jumpserver: Jumpserver) -> Self {
        Self {
            clap_command: command,
            jumpserver,
        }
    }

    pub async fn run(&self) -> Result<(), error::WsshError> {
        let matches = self.clap_command.clone().get_matches();
        let env = matches.get_one::<String>("env").expect("请输入--env参数");
        let app_list = self
            .jumpserver
            .query_app_list(env.as_str())
            .map_err(|e| error::from_string(format!("获取应用列表失败: {}", e.to_string())))?;

        loop {
            let app_index = self.show_app_selections(&app_list)?;
            if app_index == 0 {
                break;
            }

            let app = app_list.get(app_index - 1).unwrap();

            let pod_list = self
                .jumpserver
                .query_pod_list(app.app_id, env.as_str())
                .map_err(|e| error::from_string(format!("获取Pod列表失败: {}", e.to_string())))?;

            let pod_list_len = pod_list.len();
            if pod_list_len == 0 {
                println!("没有可用的Pod");
                continue;
            }

            let pod;
            if pod_list_len == 1 {
                // 直接连接到Pod
                pod = pod_list.get(0).unwrap();
            } else {
                let pod_index = self.show_pod_selections(&pod_list).unwrap();
                if pod_index == 0 {
                    continue;
                }
                pod = pod_list.get(pod_index - 1).unwrap()
            }

            let _ = terminal::login(pod.ssh_token.as_str()).await;
        }

        Ok(())
    }

    fn show_app_selections(&self, app_list: &Vec<AppItem>) -> Result<usize, error::WsshError> {
        let app_selections: Vec<String> = app_list
            .iter()
            .enumerate()
            .map(|(index, app)| format!("{}. {} {}", index + 1, app.app_key, app.app_name,))
            .collect();

        let app_index =
            dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("请选择应用")
                .item("0. 退出")
                .items(&app_selections)
                .default(0)
                .interact()
                .map_err(|e| error::from_string(format!("选择应用失败: {}", e.to_string())))?;

        Ok(app_index)
    }

    fn show_pod_selections(&self, pod_list: &Vec<Pod>) -> Result<usize, error::WsshError> {
        let pod_selections: Vec<String> = pod_list
            .iter()
            .enumerate()
            .map(|(index, pod)| format!("{}. {}", index + 1, pod.pod_ip,))
            .collect();

        let pod_index =
            dialoguer::FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("请选择Pod")
                .item("0. 返回上一层")
                .items(&pod_selections)
                .interact()
                .map_err(|e| error::from_string(format!("选择Pod失败: {}", e.to_string())))?;

        Ok(pod_index)
    }
}
