/*
 * 动作执行模块 | Action Execution Module
 *
 * 功能概述 | Overview:
 * 本模块实现 URL Dispatcher 支持的所有动作的实际执行逻辑，包括：
 * - 剪贴板操作（使用 arboard crate 跨平台访问系统剪贴板）
 * - 文件追加操作（带时间戳记录 URL 到文件）
 * - 浏览器启动操作（使用指定可执行文件和参数打开 URL）
 *
 * This module implements the actual execution logic for all actions supported
 * by URL Dispatcher, including:
 * - Clipboard operations (using arboard crate for cross-platform system clipboard access)
 * - File append operations (recording URLs to file with timestamps)
 * - Browser launch operations (opening URLs with specified executable and arguments)
 *
 * 设计说明 | Design Notes:
 * - 所有函数返回 Result<()>，统一错误处理
 * - 使用 arboard 实现跨平台剪贴板访问，支持 Windows/Linux/macOS
 * - 文件操作使用追加模式，避免覆盖现有内容
 * - 浏览器启动使用 spawn 而非 wait，立即返回不阻塞
 *
 * - All functions return Result<()> for unified error handling
 * - Uses arboard for cross-platform clipboard access, supporting Windows/Linux/macOS
 * - File operations use append mode to avoid overwriting existing content
 * - Browser launch uses spawn instead of wait, returns immediately without blocking
 */

use anyhow::{anyhow, Result};
use std::path::Path;

/// 复制 URL 到系统剪贴板 | Copy URL to system clipboard
///
/// 使用 arboard crate 访问系统剪贴板，支持 Windows/Linux/macOS 三大平台。
/// 在 Linux 上需要 X11 或 Wayland 环境。
///
/// Uses arboard crate to access system clipboard, supporting Windows/Linux/macOS.
/// On Linux, requires X11 or Wayland environment.
///
/// 参数 | Parameters:
/// - `url`: 要复制到剪贴板的 URL 字符串
///         URL string to copy to clipboard
///
/// 返回值 | Return:
/// 成功返回 Ok(())，失败返回包含错误详情的 Err。
/// Returns Ok(()) on success, Err with error details on failure.
///
/// 关键步骤 | Key Steps:
/// 1. 创建剪贴板句柄
///    Create clipboard handle
/// 2. 将文本设置到剪贴板
///    Set text to clipboard
pub fn copy_to_clipboard(url: &str) -> Result<()> {
    // 创建 arboard 剪贴板实例，自动处理平台差异
    // Create arboard clipboard instance, automatically handles platform differences
    let mut clipboard = arboard::Clipboard::new()?;
    // 设置文本内容到剪贴板
    // Set text content to clipboard
    clipboard.set_text(url)?;
    Ok(())
}

/// 将 URL（带时间戳）追加到指定文件 | Append URL with timestamp to specified file
///
/// 以追加模式打开文件，在每行前添加时间戳。如果文件或父目录不存在，
/// 会自动创建。使用追加模式确保不会覆盖现有内容。
///
/// Opens file in append mode, adding timestamp before each line. Automatically
/// creates file and parent directories if they don't exist. Append mode ensures
/// existing content won't be overwritten.
///
/// 参数 | Parameters:
/// - `url`: 要追加的 URL 字符串
///         URL string to append
/// - `file_path`: 目标文件路径
///               Target file path
///
/// 返回值 | Return:
/// 成功返回 Ok(())，失败返回错误。
/// Returns Ok(()) on success, error on failure.
///
/// 格式 | Format:
/// 每行格式为：[YYYY-MM-DD HH:MM:SS] URL
/// Each line format: [YYYY-MM-DD HH:MM:SS] URL
///
/// 时间戳格式 | Timestamp Format:
/// - 使用本地时区
///   Uses local timezone
/// - 24 小时制
///   24-hour format
/// - 精确到秒
///   Accurate to seconds
///
/// 关键步骤 | Key Steps:
/// 1. 确保父目录存在（自动创建）
///    Ensure parent directory exists (auto-create)
/// 2. 以追加模式打开文件（不存在则创建）
///    Open file in append mode (create if not exists)
/// 3. 获取当前时间戳
///    Get current timestamp
/// 4. 写入格式化的行
///    Write formatted line
pub fn append_to_file(url: &str, file_path: &Path) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    // 确保父目录存在 | Ensure parent directory exists
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // 以追加模式打开文件，不存在则创建
    // Open file in append mode, create if not exists
    let mut file = OpenOptions::new()
        .create(true) // 文件不存在时创建 | Create if doesn't exist
        .append(true) // 追加模式，不覆盖现有内容 | Append mode, don't overwrite
        .open(file_path)?;

    // 生成本地时间戳，格式：YYYY-MM-DD HH:MM:SS
    // Generate local timestamp, format: YYYY-MM-DD HH:MM:SS
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    // 写入格式化的行：[时间戳] URL
    // Write formatted line: [timestamp] URL
    writeln!(file, "[{}] {}", timestamp, url)?;
    Ok(())
}

/// 用指定浏览器打开 URL | Open URL in specified browser
///
/// 使用指定的可执行文件和命令行参数打开 URL。支持 {URL} 占位符，
/// 会在执行前替换为实际 URL。使用 spawn 方式启动进程，不等待浏览器退出。
///
/// Opens URL using specified executable and command line arguments. Supports
/// {URL} placeholder which will be replaced with actual URL before execution.
/// Uses spawn to start process without waiting for browser to exit.
///
/// 参数 | Parameters:
/// - `url`: 要打开的 URL 字符串
///         URL string to open
/// - `executable`: 浏览器可执行文件路径（如 /usr/bin/firefox）
///                Browser executable path (e.g., /usr/bin/firefox)
/// - `args_template`: 命令行参数模板数组，可包含 {URL} 占位符
///                   Command line arguments template array, may contain {URL} placeholder
///
/// 返回值 | Return:
/// 成功返回 Ok(())，失败返回包含可执行文件名和错误信息的错误。
/// Returns Ok(()) on success, error containing executable name and error details on failure.
///
/// {URL} 占位符机制 | {URL} Placeholder Mechanism:
/// - 参数中的所有 {URL} 都会被替换为实际 URL
///   All {URL} in arguments will be replaced with actual URL
/// - 如果参数为空，默认传递 URL 作为唯一参数
///   If arguments are empty, URL is passed as the only argument by default
/// - 示例：["--incognito", "{URL}"] -> ["--incognito", "https://example.com"]
///   Example: ["--incognito", "{URL}"] -> ["--incognito", "https://example.com"]
///
/// 进程生成 | Process Spawning:
/// - 使用 spawn 而非 wait，立即返回
///   Uses spawn instead of wait, returns immediately
/// - 不等待浏览器进程结束
///   Doesn't wait for browser process to end
/// - 浏览器进程独立运行
///   Browser process runs independently
///
/// 关键步骤 | Key Steps:
/// 1. 处理参数模板：空则使用 URL，否则替换 {URL} 占位符
///    Process argument template: use URL if empty, otherwise replace {URL} placeholder
/// 2. 生成子进程，传递可执行文件和参数
///    Spawn child process with executable and arguments
/// 3. 检查启动结果，失败则返回友好错误信息
///    Check launch result, return friendly error message on failure
pub fn open_in_browser(url: &str, executable: &str, args_template: &[String]) -> Result<()> {
    // 处理参数：空数组则默认传递 URL，否则替换 {URL} 占位符
    // Process arguments: default to URL if empty array, otherwise replace {URL} placeholder
    let args: Vec<String> = if args_template.is_empty() {
        vec![url.to_string()]
    } else {
        args_template
            .iter()
            .map(|arg| arg.replace("{URL}", url))
            .collect()
    };

    // 生成子进程，不等待其结束
    // Spawn child process without waiting for it to finish
    let child = std::process::Command::new(executable).args(&args).spawn();

    // 检查启动结果
    // Check launch result
    match child {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to launch '{}': {}", executable, e)),
    }
}
