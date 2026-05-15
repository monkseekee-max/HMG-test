use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    Zh,
    En,
}

impl Lang {
    fn code(self) -> &'static str {
        match self {
            Lang::Zh => "zh-CN",
            Lang::En => "en",
        }
    }
}

#[derive(Clone, Copy)]
struct SiteCopy {
    nav_label: &'static str,
    nav_model: &'static str,
    nav_trust: &'static str,
    nav_quickstart: &'static str,
    language_label: &'static str,
    hero_eyebrow: &'static str,
    hero_lede: &'static str,
    hero_primary: &'static str,
    hero_secondary: &'static str,
    hero_actions_label: &'static str,
    problem_kicker: &'static str,
    problem_title: &'static str,
    problem_one: &'static str,
    problem_two: &'static str,
    problem_three: &'static str,
    model_kicker: &'static str,
    model_title: &'static str,
    model_body: &'static str,
    explorer_label: &'static str,
    metric_label: &'static str,
    trust_kicker: &'static str,
    trust_title: &'static str,
    trust_scope_label: &'static str,
    trust_scope_title: &'static str,
    trust_scope_body: &'static str,
    trust_repair_label: &'static str,
    trust_repair_title: &'static str,
    trust_repair_body: &'static str,
    trust_audit_label: &'static str,
    trust_audit_title: &'static str,
    trust_audit_body: &'static str,
    quickstart_kicker: &'static str,
    quickstart_title: &'static str,
    quickstart_body: &'static str,
    quickstart_code_label: &'static str,
    final_kicker: &'static str,
    final_title: &'static str,
    final_button: &'static str,
    footer_body: &'static str,
}

#[derive(Clone, Copy)]
struct MemoryLayer {
    name_zh: &'static str,
    name_en: &'static str,
    label_zh: &'static str,
    label_en: &'static str,
    detail_zh: &'static str,
    detail_en: &'static str,
    metric_zh: &'static str,
    metric_en: &'static str,
}

#[derive(Clone, Copy)]
struct ValuePillar {
    title_zh: &'static str,
    title_en: &'static str,
    body_zh: &'static str,
    body_en: &'static str,
    metric_zh: &'static str,
    metric_en: &'static str,
}

#[derive(Clone, Copy)]
struct ProductDocTopic {
    id: &'static str,
    label: &'static str,
    title_zh: &'static str,
    title_en: &'static str,
    summary_zh: &'static str,
    summary_en: &'static str,
    bullets_zh: &'static [&'static str],
    bullets_en: &'static [&'static str],
    code: &'static str,
}

impl MemoryLayer {
    fn name(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.name_zh,
            Lang::En => self.name_en,
        }
    }

    fn label(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.label_zh,
            Lang::En => self.label_en,
        }
    }

    fn detail(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.detail_zh,
            Lang::En => self.detail_en,
        }
    }

    fn metric(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.metric_zh,
            Lang::En => self.metric_en,
        }
    }
}

impl ProductDocTopic {
    fn title(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.title_zh,
            Lang::En => self.title_en,
        }
    }

    fn summary(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.summary_zh,
            Lang::En => self.summary_en,
        }
    }

    fn bullets(self, lang: Lang) -> &'static [&'static str] {
        match lang {
            Lang::Zh => self.bullets_zh,
            Lang::En => self.bullets_en,
        }
    }
}

impl ValuePillar {
    fn title(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.title_zh,
            Lang::En => self.title_en,
        }
    }

    fn body(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.body_zh,
            Lang::En => self.body_en,
        }
    }

    fn metric(self, lang: Lang) -> &'static str {
        match lang {
            Lang::Zh => self.metric_zh,
            Lang::En => self.metric_en,
        }
    }
}

const VALUE_PILLARS: [ValuePillar; 4] = [
    ValuePillar {
        title_zh: "少提示也能记得",
        title_en: "Memory without prompt tax",
        body_zh: "用 AGENTS.md + memory_agent_brief 把召回变成 Codex 的默认开场，而不是用户反复提醒。",
        body_en: "AGENTS.md plus memory_agent_brief turns recall into the default Codex opening, not a repeated user reminder.",
        metric_zh: "启动即简报",
        metric_en: "brief on start",
    },
    ValuePillar {
        title_zh: "按分支保留判断力",
        title_en: "Branch-scoped judgment",
        body_zh: "把决策、根因、风险和被拒绝方案绑定到 workspace / repo / branch，避免跨任务污染。",
        body_en: "Decisions, root causes, risks, and rejected paths stay scoped to workspace / repo / branch to avoid cross-task pollution.",
        metric_zh: "作用域优先",
        metric_en: "scope first",
    },
    ValuePillar {
        title_zh: "错记忆可纠正",
        title_en: "Correctable memory",
        body_zh: "旧事实不会被覆盖成黑箱；correction、governance 和 history 保留完整追溯链。",
        body_en: "Old facts are not overwritten into a black box; correction, governance, and history preserve the chain.",
        metric_zh: "可审计",
        metric_en: "auditable",
    },
    ValuePillar {
        title_zh: "交接形成闭环",
        title_en: "Handoff closes the loop",
        body_zh: "任务结束用 memory_handoff 写回验证、风险和下一步，让下一次会话直接接上。",
        body_en: "memory_handoff writes back validation, risk, and next steps so the next session can continue.",
        metric_zh: "跨会话连续",
        metric_en: "cross-session",
    },
];

const MEMORY_LAYERS: [MemoryLayer; 4] = [
    MemoryLayer {
        name_zh: "Agent 简报",
        name_en: "Agent brief",
        label_zh: "进入",
        label_en: "entry",
        detail_zh: "每次会话开始前，先取回当前分支的决策、根因、约束、风险和下一步。",
        detail_en: "Starts each session with branch-aware decisions, root causes, constraints, risks, and next steps.",
        metric_zh: "编辑前",
        metric_en: "before edit",
    },
    MemoryLayer {
        name_zh: "召回图谱",
        name_en: "Recall graph",
        label_zh: "召回",
        label_en: "retrieve",
        detail_zh: "用图谱关系、来源线索和生命周期状态排序记忆，避免陈旧事实意外获胜。",
        detail_en: "Ranks memories with graph context, source hints, and lifecycle state so stale facts do not win by accident.",
        metric_zh: "混合召回",
        metric_en: "hybrid recall",
    },
    MemoryLayer {
        name_zh: "纠错",
        name_en: "Correction",
        label_zh: "修复",
        label_en: "repair",
        detail_zh: "冲突事实会被纠正、降权或治理，而不是变成一堆看不见的矛盾。",
        detail_en: "Conflicting facts are corrected, demoted, or governed instead of piling up as invisible contradiction.",
        metric_zh: "不静默漂移",
        metric_en: "no silent drift",
    },
    MemoryLayer {
        name_zh: "交接",
        name_en: "Handoff",
        label_zh: "退出",
        label_en: "exit",
        detail_zh: "记录改了什么、为什么改、验证了什么，以及下一个 agent 必须知道的风险。",
        detail_en: "Captures what changed, why it changed, what was verified, and what remains risky for the next agent.",
        metric_zh: "验证后",
        metric_en: "after verify",
    },
];

const QUICKSTART_CODE: &str = r#"curl -fsSL https://funcode.xin/HMG/install.sh | sh
hmg init -g
hmg doctor
codex"#;

const CODEX_MCP_CONFIG: &str = r#"[mcp_servers.hmg]
type = "stdio"
command = "/home/USER/.local/bin/hmg-server"
args = ["/home/USER/.local/share/hmg/codex-data"]
startup_timeout_sec = 30
env = { HMG_PROVIDER_BACKEND = "local" }"#;

const PRODUCT_DOC_TOPICS: [ProductDocTopic; 8] = [
    ProductDocTopic {
        id: "install-doc",
        label: "01",
        title_zh: "安装与初始化",
        title_en: "Install and initialize",
        summary_zh: "HMG 官网安装脚本会优先下载 GitHub Release 二进制；如果当前平台还没有预编译包，会回退到 cargo 从 GitHub 安装。",
        summary_en: "The HMG installer prefers GitHub Release binaries and falls back to cargo install from GitHub when a prebuilt package is not available.",
        bullets_zh: &[
            "安装后会提供 hmg、hmg-server、hmg-hook-worker 三个命令。",
            "hmg init 在当前目录 AGENTS.md 头部写入 HMG 自动记忆策略。",
            "hmg init -g 写入 ~/.codex/AGENTS.md，让所有 Codex 工作区默认生效。",
            "hmg doctor 用于验证命令、全局策略和 Codex MCP 配置是否就绪。",
        ],
        bullets_en: &[
            "Installation provides hmg, hmg-server, and hmg-hook-worker.",
            "hmg init writes the HMG memory policy into the current AGENTS.md.",
            "hmg init -g writes ~/.codex/AGENTS.md so every Codex workspace can inherit it.",
            "hmg doctor verifies commands, global policy, and Codex MCP readiness.",
        ],
        code: QUICKSTART_CODE,
    },
    ProductDocTopic {
        id: "codex-doc",
        label: "02",
        title_zh: "Codex MCP 配置",
        title_en: "Codex MCP configuration",
        summary_zh: "Codex 通过 stdio MCP 启动 hmg-server。不要手动启动 HTTP 服务，也不要在 MCP args 里加 --http。",
        summary_en: "Codex launches hmg-server through stdio MCP. Do not start an HTTP server manually and do not add --http to MCP args.",
        bullets_zh: &[
            "command 指向 hmg-server 的绝对路径。",
            "args 是持久化数据目录，建议放在 ~/.local/share/hmg/codex-data。",
            "HMG_PROVIDER_BACKEND=local 适合确定性本地 smoke；生产可切换为 openai、anthropic 或 compatible。",
        ],
        bullets_en: &[
            "command points to the absolute hmg-server path.",
            "args is the persistent data directory, usually ~/.local/share/hmg/codex-data.",
            "HMG_PROVIDER_BACKEND=local is deterministic for local smoke; production can use openai, anthropic, or compatible.",
        ],
        code: CODEX_MCP_CONFIG,
    },
    ProductDocTopic {
        id: "loop-doc",
        label: "03",
        title_zh: "自动记忆工作流",
        title_en: "Autonomous memory loop",
        summary_zh: "HMG 的目标不是让用户反复提示“调用 MCP”，而是让 Codex 在任务开始、编辑前、发现 durable 信息、任务结束时自然使用记忆。",
        summary_en: "HMG should not require users to repeatedly say “call MCP”; Codex should naturally use memory at task start, before edits, when durable facts appear, and at task end.",
        bullets_zh: &[
            "任务开始：memory_agent_brief 拉取分支简报。",
            "编辑前：召回相关决策、根因、拒绝方案和治理记录。",
            "任务结束：memory_handoff 写入验证、风险和下一步。",
        ],
        bullets_en: &[
            "Task start: memory_agent_brief retrieves a branch-aware brief.",
            "Before edits: recall decisions, root causes, rejected paths, and governance records.",
            "Task end: memory_handoff writes validation, risks, and next steps.",
        ],
        code: "hmg init -g\nhmg doctor\ncodex",
    },
    ProductDocTopic {
        id: "tools-doc",
        label: "04",
        title_zh: "MCP 工具参考",
        title_en: "MCP tool reference",
        summary_zh: "HMG 暴露高层 agent 工具和底层记忆工具。推荐 Codex 默认使用高层 brief / handoff，深入排查时再使用 recall / history。",
        summary_en: "HMG exposes high-level agent tools and lower-level memory tools. Codex should prefer brief / handoff by default and use recall / history for deeper inspection.",
        bullets_zh: &[
            "memory_agent_brief：任务开始简报。",
            "memory_handoff：任务结束交接。",
            "memory_recall、memory_correct、memory_govern、memory_history：召回、纠错、治理、审计。",
        ],
        bullets_en: &[
            "memory_agent_brief: task-start brief.",
            "memory_handoff: task-end handoff.",
            "memory_recall, memory_correct, memory_govern, memory_history: recall, correction, governance, audit.",
        ],
        code: "memory_agent_brief\nmemory_handoff\nmemory_recall\nmemory_correct\nmemory_govern\nmemory_history",
    },
    ProductDocTopic {
        id: "governance-doc",
        label: "05",
        title_zh: "治理、纠错与审计",
        title_en: "Governance, correction, and audit",
        summary_zh: "HMG 不覆盖旧事实，而是追加纠错和治理记录；正常召回、治理召回、审计召回可以看到不同视图。",
        summary_en: "HMG does not overwrite old facts; it appends correction and governance records. Normal, governance, and audit recall can expose different views.",
        bullets_zh: &[
            "错误事实用 memory_correct replace / negate 处理。",
            "敏感内容用 memory_govern seal / tombstone 进入受控历史。",
            "memory_history 展示 supersedes、derived lesson 和 exposure history。",
        ],
        bullets_en: &[
            "Wrong facts use memory_correct replace / negate.",
            "Sensitive content uses memory_govern seal / tombstone for controlled history.",
            "memory_history shows supersedes, derived lesson, and exposure history.",
        ],
        code: "memory_correct(action=\"replace\")\nmemory_govern(action=\"seal\")\nmemory_history(atom_id=\"...\")",
    },
    ProductDocTopic {
        id: "troubleshooting-doc",
        label: "06",
        title_zh: "排障：MCP 启动失败",
        title_en: "Troubleshooting MCP startup",
        summary_zh: "Codex MCP stdio 要求 stdout 只能输出 JSON-RPC。HMG 已把日志固定写到 stderr，并且 notification 不回包。",
        summary_en: "Codex MCP stdio requires stdout to contain only JSON-RPC. HMG writes logs to stderr and does not respond to notifications.",
        bullets_zh: &[
            "如果看到 Transport closed，先确认 command 指向最新 hmg-server。",
            "运行 codex mcp list / codex mcp get hmg 检查配置。",
            "用 scripts/run-hmg-codex-real-scenario.sh 做确定性 smoke。",
        ],
        bullets_en: &[
            "If Transport closed appears, confirm command points to the latest hmg-server.",
            "Run codex mcp list / codex mcp get hmg to inspect config.",
            "Use scripts/run-hmg-codex-real-scenario.sh as the deterministic smoke path.",
        ],
        code: "codex mcp list\ncodex mcp get hmg\nbash scripts/run-hmg-codex-real-scenario.sh",
    },
    ProductDocTopic {
        id: "release-doc",
        label: "07",
        title_zh: "发布与更新路径",
        title_en: "Release and update path",
        summary_zh: "更新不再要求用户复制 cargo install 命令；统一使用 hmg update。它默认走官网 install.sh，未来有 release 二进制时会自动优先下载。",
        summary_en: "Updates no longer require users to copy cargo install commands; use hmg update. It defaults to the website installer and will prefer release binaries when they are available.",
        bullets_zh: &[
            "日常更新：hmg update。",
            "验证更新：hmg doctor。",
            "开发或救援场景：hmg update --git <repo> 仍可从源码安装。",
        ],
        bullets_en: &[
            "Daily update: hmg update.",
            "Verify the update: hmg doctor.",
            "Development or rescue path: hmg update --git <repo> can still install from source.",
        ],
        code: "hmg update\nhmg doctor\nhmg update --dry-run",
    },
    ProductDocTopic {
        id: "cli-tui-doc",
        label: "08",
        title_zh: "命令行与 TUI 能力",
        title_en: "CLI and TUI capabilities",
        summary_zh: "hmg CLI 正在从初始化脚本升级为稳定的本地控制面：初始化、更新、诊断、版本和轻量控制台都有明确命令。",
        summary_en: "The hmg CLI is maturing from an initializer into a local control plane with explicit commands for init, update, diagnostics, versioning, and a lightweight console.",
        bullets_zh: &[
            "hmg --help / hmg version：基础可发现性。",
            "hmg init / hmg init -g：本地或全局注入自动记忆策略。",
            "hmg doctor / hmg tui：检查 server、Codex、AGENTS.md 和 MCP 配置状态。",
        ],
        bullets_en: &[
            "hmg --help / hmg version: basic discoverability.",
            "hmg init / hmg init -g: install local or global autonomous memory policy.",
            "hmg doctor / hmg tui: inspect server, Codex, AGENTS.md, and MCP configuration readiness.",
        ],
        code: "hmg --help\nhmg version\nhmg doctor\nhmg tui",
    },
];

fn copy(lang: Lang) -> SiteCopy {
    match lang {
        Lang::Zh => SiteCopy {
            nav_label: "页面导航",
            nav_model: "模型",
            nav_trust: "信任",
            nav_quickstart: "快速开始",
            language_label: "语言切换",
            hero_eyebrow: "面向自主编码 agent 的分支感知记忆",
            hero_lede: "HMG 是可信的记忆图谱，帮助 agent 在改代码前记住决策、被拒绝的方案、根因和验证历史。",
            hero_primary: "立即安装",
            hero_secondary: "查看文档中心",
            hero_actions_label: "主要操作",
            problem_kicker: "失效模式",
            problem_title: "Agent 失败不是因为 token 不够，而是因为判断力丢了。",
            problem_one: "下一次会话会重复上一轮已经否决过的方案。",
            problem_two: "陈旧记忆会压过已纠正事实，因为它们看起来都是普通文本。",
            problem_three: "验证结果从交接里消失，信心就变成了表演。",
            model_kicker: "模型",
            model_title: "一种按工程工作方式组织的记忆系统。",
            model_body: "HMG 保存一次编码会话真正有用的部分：决策为什么发生、试过什么、失败了什么、验证了什么，以及下一个 agent 不能忘什么。",
            explorer_label: "当前层",
            metric_label: "状态",
            trust_kicker: "信任层",
            trust_title: "记忆需要治理，而不是感觉。",
            trust_scope_label: "作用域",
            trust_scope_title: "默认感知分支",
            trust_scope_body: "某个分支上的高风险实验，不应该自动变成所有后续 agent 的全局建议。",
            trust_repair_label: "修复",
            trust_repair_title: "纠正旧事实",
            trust_repair_body: "用纠正、降权和治理处理过时信息，而不是写一段更新的文字让冲突悄悄存在。",
            trust_audit_label: "审计",
            trust_audit_title: "让链路可读",
            trust_audit_body: "有用的交接应该带上来源、验证、信心、风险，以及塑造它的决策。",
            quickstart_kicker: "快速开始",
            quickstart_title: "一条命令安装，下一条命令让 Codex 自动记忆。",
            quickstart_body: "官网安装脚本会优先下载 release 二进制；如果还没有匹配包，会回退到 cargo 从 GitHub 安装。随后用 hmg init -g 写入全局 Codex 记忆策略。",
            quickstart_code_label: "HMG 安装命令",
            final_kicker: "给正在用 agent 交付软件的团队",
            final_title: "把你希望上一次会话保留下来的上下文，交给下一次会话。",
            final_button: "接入 HMG 循环",
            footer_body: "面向自主工程系统的全息记忆图谱。",
        },
        Lang::En => SiteCopy {
            nav_label: "Primary navigation",
            nav_model: "Model",
            nav_trust: "Trust",
            nav_quickstart: "Quickstart",
            language_label: "Language switcher",
            hero_eyebrow: "Branch-aware memory for autonomous coding agents",
            hero_lede: "A trustworthy memory graph that helps agents remember decisions, rejected paths, root causes, and verification history before they touch the code.",
            hero_primary: "Install now",
            hero_secondary: "Read the docs",
            hero_actions_label: "Primary actions",
            problem_kicker: "The failure mode",
            problem_title: "Agents do not fail because they lack tokens. They fail because they lose judgment.",
            problem_one: "The next session repeats an option the last session already rejected.",
            problem_two: "A stale memory outranks a corrected one because both look like plain text.",
            problem_three: "Verification disappears from the handoff, so confidence becomes theater.",
            model_kicker: "The model",
            model_title: "A memory system shaped like engineering work.",
            model_body: "HMG stores the useful parts of a coding session: why a decision happened, what was tried, what failed, what was verified, and what the next agent must not forget.",
            explorer_label: "Current layer",
            metric_label: "state",
            trust_kicker: "Trust layer",
            trust_title: "Memory needs governance, not vibes.",
            trust_scope_label: "scope",
            trust_scope_title: "Branch-aware by default",
            trust_scope_body: "A risky experiment on one branch should not become global advice for every future agent.",
            trust_repair_label: "repair",
            trust_repair_title: "Correct old facts",
            trust_repair_body: "Use correction, demotion, and governance instead of writing a newer paragraph that quietly conflicts.",
            trust_audit_label: "audit",
            trust_audit_title: "Keep the chain readable",
            trust_audit_body: "Every useful handoff carries source, validation, confidence, risk, and the decision that shaped it.",
            quickstart_kicker: "Quickstart",
            quickstart_title: "Install with one command. Let Codex remember with the next one.",
            quickstart_body: "The installer prefers release binaries and falls back to cargo from GitHub. Then hmg init -g writes the global Codex memory policy.",
            quickstart_code_label: "HMG install command",
            final_kicker: "For builders shipping with agents",
            final_title: "Give the next session the context you wish the last one had kept.",
            final_button: "Wire HMG into the loop",
            footer_body: "Holographic Memory Graph for autonomous engineering systems.",
        },
    }
}

fn main() {
    leptos::mount::mount_to_body(|| {
        view! { <App /> }
    });
}

#[component]
fn App() -> impl IntoView {
    let (lang, set_lang) = signal(Lang::Zh);

    view! {
        <div class="site-shell" lang=move || lang.get().code()>
            <SiteHeader lang set_lang />
            <main>
                <Hero lang />
                <QuickstartSection lang />
                <ProblemSection lang />
                <ModelSection lang />
                <GovernanceSection lang />
                <DocsSection lang />
                <FinalCta lang />
            </main>
            <SiteFooter lang />
        </div>
    }
}

#[component]
fn SiteHeader(lang: ReadSignal<Lang>, set_lang: WriteSignal<Lang>) -> impl IntoView {
    view! {
        <header class="site-header" aria-label=move || copy(lang.get()).nav_label>
            <a class="brand-lockup" href="#top" aria-label="HMG home">
                <span class="brand-mark" aria-hidden="true"></span>
                <span>HMG</span>
            </a>
            <div class="header-right">
                <nav class="nav-links" aria-label=move || copy(lang.get()).nav_label>
                    <a href="#model">{move || copy(lang.get()).nav_model}</a>
                    <a href="#trust">{move || copy(lang.get()).nav_trust}</a>
                    <a href="#quickstart">{move || copy(lang.get()).nav_quickstart}</a>
                    <a href="#docs">{move || match lang.get() { Lang::Zh => "文档", Lang::En => "Docs" }}</a>
                </nav>
                <div class="language-toggle" aria-label=move || copy(lang.get()).language_label>
                    <button
                        type="button"
                        class:selected=move || lang.get() == Lang::Zh
                        aria-pressed=move || lang.get() == Lang::Zh
                        on:click=move |_| set_lang.set(Lang::Zh)
                    >
                        "中"
                    </button>
                    <button
                        type="button"
                        class:selected=move || lang.get() == Lang::En
                        aria-pressed=move || lang.get() == Lang::En
                        on:click=move |_| set_lang.set(Lang::En)
                    >
                        "EN"
                    </button>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Hero(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <section class="hero" id="top">
            <MemoryScene lang />
            <div class="hero-copy">
                <p class="eyebrow">{move || copy(lang.get()).hero_eyebrow}</p>
                <h1>"HMG"</h1>
                <p class="hero-lede">{move || copy(lang.get()).hero_lede}</p>
                <div class="hero-actions" aria-label=move || copy(lang.get()).hero_actions_label>
                    <a class="button primary" href="#quickstart">
                        {move || copy(lang.get()).hero_primary}
                    </a>
                    <a class="button secondary" href="#docs">
                        {move || copy(lang.get()).hero_secondary}
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn MemoryScene(lang: ReadSignal<Lang>) -> impl IntoView {
    let brief_label = move || match lang.get() {
        Lang::Zh => "会话简报",
        Lang::En => "agent brief",
    };
    let cause_label = move || match lang.get() {
        Lang::Zh => "根因",
        Lang::En => "root cause",
    };
    let cause_value = move || match lang.get() {
        Lang::Zh => "鉴权竞争",
        Lang::En => "auth race",
    };
    let rejected_label = move || match lang.get() {
        Lang::Zh => "已拒绝",
        Lang::En => "rejected",
    };
    let rejected_value = move || match lang.get() {
        Lang::Zh => "定时刷新",
        Lang::En => "timer refresh",
    };
    let risk_label = move || match lang.get() {
        Lang::Zh => "风险",
        Lang::En => "risk",
    };
    let risk_value = move || match lang.get() {
        Lang::Zh => "冷启动",
        Lang::En => "cold start",
    };
    let trace_decision = move || match lang.get() {
        Lang::Zh => "decision: 保留分支作用域",
        Lang::En => "decision: preserve branch scope",
    };
    let trace_constraint = move || match lang.get() {
        Lang::Zh => "constraint: 不允许静默矛盾",
        Lang::En => "constraint: no silent contradiction",
    };
    let trace_warning = move || match lang.get() {
        Lang::Zh => "not-tested: 生产延迟",
        Lang::En => "not-tested: production latency",
    };

    view! {
        <div class="memory-scene" aria-hidden="true">
            <div class="scene-grid"></div>
            <div class="graph-line line-a"></div>
            <div class="graph-line line-b"></div>
            <div class="graph-line line-c"></div>
            <div class="graph-node node-brief">
                <span>{brief_label}</span>
                <strong>"main"</strong>
            </div>
            <div class="graph-node node-cause">
                <span>{cause_label}</span>
                <strong>{cause_value}</strong>
            </div>
            <div class="graph-node node-rejected">
                <span>{rejected_label}</span>
                <strong>{rejected_value}</strong>
            </div>
            <div class="graph-node node-risk">
                <span>{risk_label}</span>
                <strong>{risk_value}</strong>
            </div>
            <div class="trace-plane">
                <div class="trace-line hot">"memory_agent_brief() -> 8 atoms"</div>
                <div class="trace-line">{trace_decision}</div>
                <div class="trace-line">{trace_constraint}</div>
                <div class="trace-line warn">{trace_warning}</div>
            </div>
        </div>
    }
}

#[component]
fn ProblemSection(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <section class="section problem-section" aria-labelledby="problem-title">
            <div class="section-kicker">{move || copy(lang.get()).problem_kicker}</div>
            <div class="problem-layout">
                <h2 id="problem-title">{move || copy(lang.get()).problem_title}</h2>
                <div class="problem-list">
                    <div class="problem-row">
                        <span>"01"</span>
                        <p>{move || copy(lang.get()).problem_one}</p>
                    </div>
                    <div class="problem-row">
                        <span>"02"</span>
                        <p>{move || copy(lang.get()).problem_two}</p>
                    </div>
                    <div class="problem-row">
                        <span>"03"</span>
                        <p>{move || copy(lang.get()).problem_three}</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

#[component]
fn ModelSection(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <section class="section model-section" id="model" aria-labelledby="model-title">
            <div class="model-heading">
                <div>
                    <div class="section-kicker">{move || copy(lang.get()).model_kicker}</div>
                    <h2 id="model-title">{move || copy(lang.get()).model_title}</h2>
                </div>
                <p>{move || copy(lang.get()).model_body}</p>
            </div>
            <ProductValueShowcase lang />
            <MemoryExplorer lang />
        </section>
    }
}

#[component]
fn ProductValueShowcase(lang: ReadSignal<Lang>) -> impl IntoView {
    let value_label = move || match lang.get() {
        Lang::Zh => "核心价值",
        Lang::En => "core value",
    };
    let motion_label = move || match lang.get() {
        Lang::Zh => "Remotion 风格的记忆循环",
        Lang::En => "Remotion-style memory loop",
    };
    let frame_brief = move || match lang.get() {
        Lang::Zh => "读取分支简报",
        Lang::En => "read branch brief",
    };
    let frame_act = move || match lang.get() {
        Lang::Zh => "带上下文修改",
        Lang::En => "edit with context",
    };
    let frame_repair = move || match lang.get() {
        Lang::Zh => "纠正旧事实",
        Lang::En => "correct stale fact",
    };
    let frame_handoff = move || match lang.get() {
        Lang::Zh => "写回交接",
        Lang::En => "write handoff",
    };

    view! {
        <div class="value-showcase">
            <div class="value-grid" aria-label=value_label>
                <ValueCard pillar=VALUE_PILLARS[0] lang />
                <ValueCard pillar=VALUE_PILLARS[1] lang />
                <ValueCard pillar=VALUE_PILLARS[2] lang />
                <ValueCard pillar=VALUE_PILLARS[3] lang />
            </div>
            <div class="remotion-stage" aria-label=motion_label>
                <div class="remotion-orbit" aria-hidden="true">
                    <span class="orbit-dot dot-a"></span>
                    <span class="orbit-dot dot-b"></span>
                    <span class="orbit-dot dot-c"></span>
                </div>
                <div class="remotion-header">
                    <span>"00:00:12"</span>
                    <strong>"HMG loop"</strong>
                </div>
                <div class="remotion-timeline">
                    <span class="playhead"></span>
                    <div class="timeline-card brief">
                        <small>"01"</small>
                        <strong>{frame_brief}</strong>
                    </div>
                    <div class="timeline-card act">
                        <small>"02"</small>
                        <strong>{frame_act}</strong>
                    </div>
                    <div class="timeline-card repair">
                        <small>"03"</small>
                        <strong>{frame_repair}</strong>
                    </div>
                    <div class="timeline-card handoff">
                        <small>"04"</small>
                        <strong>{frame_handoff}</strong>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ValueCard(pillar: ValuePillar, lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <article class="value-card">
            <span>{move || pillar.metric(lang.get())}</span>
            <h3>{move || pillar.title(lang.get())}</h3>
            <p>{move || pillar.body(lang.get())}</p>
        </article>
    }
}

#[component]
fn MemoryExplorer(lang: ReadSignal<Lang>) -> impl IntoView {
    let (active, set_active) = signal(0usize);
    let active_name = move || MEMORY_LAYERS[active.get()].name(lang.get());
    let active_detail = move || MEMORY_LAYERS[active.get()].detail(lang.get());
    let active_metric = move || MEMORY_LAYERS[active.get()].metric(lang.get());

    view! {
        <div class="memory-explorer">
            <div
                class="layer-switcher"
                role="tablist"
                aria-label=move || match lang.get() {
                    Lang::Zh => "记忆生命周期",
                    Lang::En => "Memory lifecycle",
                }
            >
                <button
                    type="button"
                    role="tab"
                    class:selected=move || active.get() == 0
                    aria-selected=move || active.get() == 0
                    on:click=move |_| set_active.set(0)
                >
                    <span>{move || MEMORY_LAYERS[0].label(lang.get())}</span>
                    {move || MEMORY_LAYERS[0].name(lang.get())}
                </button>
                <button
                    type="button"
                    role="tab"
                    class:selected=move || active.get() == 1
                    aria-selected=move || active.get() == 1
                    on:click=move |_| set_active.set(1)
                >
                    <span>{move || MEMORY_LAYERS[1].label(lang.get())}</span>
                    {move || MEMORY_LAYERS[1].name(lang.get())}
                </button>
                <button
                    type="button"
                    role="tab"
                    class:selected=move || active.get() == 2
                    aria-selected=move || active.get() == 2
                    on:click=move |_| set_active.set(2)
                >
                    <span>{move || MEMORY_LAYERS[2].label(lang.get())}</span>
                    {move || MEMORY_LAYERS[2].name(lang.get())}
                </button>
                <button
                    type="button"
                    role="tab"
                    class:selected=move || active.get() == 3
                    aria-selected=move || active.get() == 3
                    on:click=move |_| set_active.set(3)
                >
                    <span>{move || MEMORY_LAYERS[3].label(lang.get())}</span>
                    {move || MEMORY_LAYERS[3].name(lang.get())}
                </button>
            </div>
            <div class="explorer-output" role="tabpanel">
                <p class="output-label">{move || copy(lang.get()).explorer_label}</p>
                <h3>{active_name}</h3>
                <p>{active_detail}</p>
                <div class="metric-strip">
                    <span>{move || copy(lang.get()).metric_label}</span>
                    <strong>{active_metric}</strong>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GovernanceSection(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <section class="section trust-section" id="trust" aria-labelledby="trust-title">
            <div class="section-kicker">{move || copy(lang.get()).trust_kicker}</div>
            <h2 id="trust-title">{move || copy(lang.get()).trust_title}</h2>
            <div class="trust-rail">
                <div class="trust-item">
                    <span>{move || copy(lang.get()).trust_scope_label}</span>
                    <h3>{move || copy(lang.get()).trust_scope_title}</h3>
                    <p>{move || copy(lang.get()).trust_scope_body}</p>
                </div>
                <div class="trust-item">
                    <span>{move || copy(lang.get()).trust_repair_label}</span>
                    <h3>{move || copy(lang.get()).trust_repair_title}</h3>
                    <p>{move || copy(lang.get()).trust_repair_body}</p>
                </div>
                <div class="trust-item">
                    <span>{move || copy(lang.get()).trust_audit_label}</span>
                    <h3>{move || copy(lang.get()).trust_audit_title}</h3>
                    <p>{move || copy(lang.get()).trust_audit_body}</p>
                </div>
            </div>
        </section>
    }
}

#[component]
fn QuickstartSection(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <section class="section quickstart-section" id="quickstart" aria-labelledby="quickstart-title">
            <div class="quickstart-copy">
                <div class="section-kicker">{move || copy(lang.get()).quickstart_kicker}</div>
                <h2 id="quickstart-title">{move || copy(lang.get()).quickstart_title}</h2>
                <p>{move || copy(lang.get()).quickstart_body}</p>
            </div>
            <div class="code-plane" aria-label=move || copy(lang.get()).quickstart_code_label>
                <pre><code>{QUICKSTART_CODE}</code></pre>
            </div>
        </section>
    }
}

#[component]
fn DocsSection(lang: ReadSignal<Lang>) -> impl IntoView {
    let docs_kicker = move || match lang.get() {
        Lang::Zh => "文档中心",
        Lang::En => "Docs center",
    };
    let docs_title = move || match lang.get() {
        Lang::Zh => "HMG 产品文档：从安装到自动记忆闭环。",
        Lang::En => "HMG product docs: from install to autonomous memory loops.",
    };
    let docs_body = move || match lang.get() {
        Lang::Zh => {
            "这里是官网内置的产品文档中心。你可以从安装开始，逐步完成 Codex MCP 配置、自动记忆策略、工具使用、治理审计和故障排查。"
        }
        Lang::En => {
            "This is the built-in product docs center. Start with installation, then configure Codex MCP, autonomous memory, tools, governance, audit, and troubleshooting."
        }
    };

    view! {
        <section class="section docs-section" id="docs" aria-labelledby="docs-title">
            <div class="docs-heading">
                <div>
                    <div class="section-kicker">{docs_kicker}</div>
                    <h2 id="docs-title">{docs_title}</h2>
                </div>
                <p>{docs_body}</p>
            </div>
            <div class="docs-layout">
                <aside class="docs-toc" aria-label=move || match lang.get() { Lang::Zh => "文档目录", Lang::En => "Docs navigation" }>
                    <p>{move || match lang.get() { Lang::Zh => "目录", Lang::En => "Contents" }}</p>
                    <a href="#install-doc">"01 · " {move || PRODUCT_DOC_TOPICS[0].title(lang.get())}</a>
                    <a href="#codex-doc">"02 · " {move || PRODUCT_DOC_TOPICS[1].title(lang.get())}</a>
                    <a href="#loop-doc">"03 · " {move || PRODUCT_DOC_TOPICS[2].title(lang.get())}</a>
                    <a href="#tools-doc">"04 · " {move || PRODUCT_DOC_TOPICS[3].title(lang.get())}</a>
                    <a href="#governance-doc">"05 · " {move || PRODUCT_DOC_TOPICS[4].title(lang.get())}</a>
                    <a href="#troubleshooting-doc">"06 · " {move || PRODUCT_DOC_TOPICS[5].title(lang.get())}</a>
                    <a href="#release-doc">"07 · " {move || PRODUCT_DOC_TOPICS[6].title(lang.get())}</a>
                    <a href="#cli-tui-doc">"08 · " {move || PRODUCT_DOC_TOPICS[7].title(lang.get())}</a>
                </aside>
                <div class="docs-manual">
                    <DocArticle topic=PRODUCT_DOC_TOPICS[0] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[1] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[2] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[3] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[4] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[5] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[6] lang />
                    <DocArticle topic=PRODUCT_DOC_TOPICS[7] lang />
                </div>
            </div>
        </section>
    }
}

#[component]
fn DocArticle(topic: ProductDocTopic, lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <article class="doc-article" id=topic.id>
            <div class="doc-article-heading">
                <span>{topic.label}</span>
                <h3>{move || topic.title(lang.get())}</h3>
            </div>
            <p>{move || topic.summary(lang.get())}</p>
            <ul>
                {move || topic
                    .bullets(lang.get())
                    .iter()
                    .map(|bullet| view! { <li>{*bullet}</li> })
                    .collect_view()}
            </ul>
            <pre class="doc-code"><code>{topic.code}</code></pre>
        </article>
    }
}

#[component]
fn FinalCta(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <section class="final-cta" aria-labelledby="final-title">
            <p class="section-kicker">{move || copy(lang.get()).final_kicker}</p>
            <h2 id="final-title">{move || copy(lang.get()).final_title}</h2>
            <a class="button primary" href="#quickstart">
                {move || copy(lang.get()).final_button}
            </a>
        </section>
    }
}

#[component]
fn SiteFooter(lang: ReadSignal<Lang>) -> impl IntoView {
    view! {
        <footer class="site-footer">
            <span>"HMG"</span>
            <p>{move || copy(lang.get()).footer_body}</p>
        </footer>
    }
}
