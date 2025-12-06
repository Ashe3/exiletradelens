import asyncio
import websockets
from websockets.exceptions import ConnectionClosedError 
import base64
import os
from pathlib import Path

# IMPORTANT: Set environment variable BEFORE importing torchfree_ocr
script_dir = Path(__file__).parent
model_dir = script_dir / '.easyocr_models'
model_dir.mkdir(exist_ok=True)

# TorchfreeOCR uses this environment variable for model storage
os.environ['TORCHFREE_OCR_MODULE_PATH'] = str(model_dir)

# Now import torchfree_ocr AFTER setting the environment variable
import torchfree_ocr

# Check what MODULE_PATH is actually being used
from torchfree_ocr import config
reader = torchfree_ocr.Reader(["en"])

async def handle_image(websocket):
	try: 
		async for message in websocket:
			try:
				image_data = base64.b64decode(message)
				result = reader.readtext(image_data, detail = 0)

				result_str = str(result)
				await websocket.send(result_str)
			except Exception as e:
				error_msg = f"Error processing image: {e}"
				print(error_msg)
				await websocket.send(error_msg)
	except ConnectionClosedError:
		pass  # Client disconnected normally
	except Exception:
		pass  # Ignore other errors

async def check_parent_alive():
	while True:
		await asyncio.sleep(5)
		if os.getppid() == 1:
			os._exit(0)

async def main():
	asyncio.create_task(check_parent_alive())
	
	async with websockets.serve(
		handle_image, 
		"localhost", 
		8765,
		max_size=5 * 1024 * 1024,
		ping_interval=None,
		ping_timeout=None,
		close_timeout=10
	):
		await asyncio.Future()

if __name__ == "__main__":
	asyncio.run(main())