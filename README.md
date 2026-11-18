# MHGU Monster Teleport / 大怪区域传送 v0.1.0

Recomp 离线单人大怪区域传送 MOD。地面待机或移动时按 **L + R + 方向键上**，前往第一个存活大型怪物所在区域的原生入口。长按只触发一次，松开后可再次使用；已经同区时不重新加载。

当前开发机键盘绑定为 **Q + E + ↑**；自定义键位时使用自己的 L、R、方向键上绑定。

## 下载与安装

- [下载 Recomp v0.1.0 安装包](https://github.com/Akumi-xu/MHGU-Monster-Teleport/releases/download/v0.1.0/MHGU_Monster_Teleport_v0.1.0-Recomp.zip)
- [v0.1.0 发布页](https://github.com/Akumi-xu/MHGU-Monster-Teleport/releases/tag/v0.1.0)

1. 在游戏目录 `mods/` 下新建 `MONSTER_TELEPORT/`。
2. 将安装包内的 `romfs` 解压到该目录，确认存在 `mods/MONSTER_TELEPORT/romfs/nativeNX/plugins/monster_teleport.nro`。
3. 在 MOD 管理中启用并重启游戏。无需额外安装 Switch 加载器，不替换 exe/dll。

卸载时退出游戏，停用或移除 `mods/MONSTER_TELEPORT/`。

## 使用范围

- 本机已验证 **MHGURecomp PC 0.17.5 / MHGU 1.4.0**，内置 Forge 1.5.1。
- 仅离线单人生效。村庄、换区加载、倒地和任务结束状态不执行传送。
- 多怪按游戏内部槽位取第一只；本版没有选怪菜单或自由选区。
- 落点是目标区域的原生入口，不是怪物脚下。怪物在加载期间换区时，松开组合键再按一次。
- 使用原生换区机制，原生两只随从跟随换区已验证；未验证三随从组合或主控猎猫。
- 未验证安卓、Eden、Switch、全部地图或真实联机。任务结束测试采用受控状态设置，详见 [VALIDATION.md](VALIDATION.md)。
- 本版没有 INI 配置项，不需要编辑存档。自由选择区域留待后续版本。

## 修改与构建

源码在仓库中展开，插件逻辑为 `src/lib.rs`。请使用纯 ASCII 的源码目录。

需要 Rust nightly（含 rust-src）、cargo-forge、ARM GCC 和 Python 3。开发验证工具链为 rustc 1.101.0-nightly (12433c307 2026-10-24)、ARM GCC 15.2.1；Rust 依赖固定 `mhgu-forge =1.5.0`，其余依赖由 `Cargo.lock` 锁定。

将 ARM GCC 的 bin 目录加入 PATH 后，在仓库根目录执行：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File ./build.ps1
```

也可以分步执行：

```powershell
cargo forge build --release
python tools/check_relocations.py monster_teleport.nro
python tools/package.py
```

产物为根目录 `monster_teleport.nro`；打包结果在 `dist/`。请保留 `.cargo/`、`link/`、目标 JSON、`build.rs`、`Cargo.toml`、`Cargo.lock` 和 `rust-toolchain.toml`。重定位检查必须通过。

## 实现与排障

通过 Forge `forge_onUpdate` 读取组合键，查询现有大型怪物槽位，向原生区域管理器提交换区请求。不安装游戏函数 Hook，不修改怪物、动作资源或存档。日志中的 `[teleport]` 会记录目的区域、同区或无存活大怪的结果。

首次发布 v0.1.0：大怪所在区域传送、组合键边沿触发、原生入口落地及离线单人门控。

## 致谢

使用 Fexty12573 的 [Forge / mhgu-forge](https://github.com/Fexty12573/forge)。NRO 重定位检查工具基于本工作区维护版本，源自 Forge PR #7 的检查思路。Codex（GPT-6）协助实现、测试与发布整理。

安装包仅含本 MOD 插件与说明；仓库不包含游戏文件、生成的反编译 C、存档或密钥。
