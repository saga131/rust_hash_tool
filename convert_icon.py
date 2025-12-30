from PIL import Image
import os

def convert_jpg_to_ico(jpg_path, ico_path):
    try:
        if not os.path.exists(jpg_path):
            print(f"错误: 找不到文件 '{jpg_path}'")
            print("请确保您已将 'submit.jpg' 保存到了项目根目录。")
            return

        img = Image.open(jpg_path)
        # 转换为 RGBA 以支持透明度（虽然 jpg 没有透明度，但 ico 通常需要）
        img = img.convert("RGBA")
        # 保存为 ico，包含多种尺寸
        img.save(ico_path, format='ICO', sizes=[(256, 256), (128, 128), (64, 64), (48, 48), (32, 32), (16, 16)])
        print(f"成功: 已将 '{jpg_path}' 转换为 '{ico_path}'")
    except ImportError:
        print("错误: 需要安装 Pillow 库。请运行 'pip install Pillow'")
    except Exception as e:
        print(f"转换失败: {e}")

if __name__ == "__main__":
    convert_jpg_to_ico("submit.jpg", "icon.ico")
