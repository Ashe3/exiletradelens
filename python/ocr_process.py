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
os.environ['TORCHFREE_OCR_MODULE_PATH'] = str(model_dir)
import torchfree_ocr

reader = None
	
async def handle_image(websocket):
	global reader
	if reader is None:
		reader = torchfree_ocr.Reader(["en"])


	try: 
		async for message in websocket:
			try:
				image_data = base64.b64decode(message)
				result = reader.readtext(image_data, detail = 0)

				result_str = str(result)
				await websocket.send(result_str)
			except Exception as e:
				error_msg = f"Error processing image: {e}"
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
	try:
		asyncio.run(main())
	except KeyboardInterrupt:
		print("Stopped with KeyboardInterrupt")