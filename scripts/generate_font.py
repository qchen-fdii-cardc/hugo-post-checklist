import os
import sys
from fontTools.ttLib import TTFont
from fontTools.subset import Subsetter, Options


def main():
    print("开始处理...")

    # 读取中文字符
    try:
        with open('src/chars.rs', 'r', encoding='utf-8') as f:
            content = f.read()
            print(f"成功读取 chars.rs，内容长度：{len(content)}")
    except Exception as e:
        print(f"读取 chars.rs 失败：{e}")
        return

    # 提取中文字符
    chars = set()
    for char in content:
        if '\u4e00' <= char <= '\u9fff':  # 中文字符范围
            chars.add(char)
    print(f"提取到 {len(chars)} 个中文字符")

    # 确保输出目录存在
    os.makedirs('fonts', exist_ok=True)

    # 使用已有的字体文件
    font_path = "fonts/霞鹜文楷.ttf"

    try:
        # 验证字体文件
        if not os.path.exists(font_path):
            print(f"错误：字体文件 {font_path} 不存在")
            return

        file_size = os.path.getsize(font_path)
        print(f"字体文件大小：{file_size} 字节")
        if file_size < 1000000:  # 字体文件通常应该大于1MB
            print("警告：字体文件大小异常，可能不完整")
            return

    except Exception as e:
        print(f"处理字体文件时出错：{e}")
        return

    try:
        # 创建字体子集
        print("开始创建字体子集...")
        font = TTFont(font_path)
        subsetter = Subsetter()
        text = ''.join(chars)
        subsetter.populate(text=text)
        subsetter.subset(font)

        # 保存子集字体
        output_path = "fonts/custom_font.ttf"
        font.save(output_path)
        print(f"字体子集创建成功：{output_path}")

    except Exception as e:
        print(f"创建字体子集时出错：{e}")
        return


if __name__ == "__main__":
    main()
