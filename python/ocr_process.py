import asyncio
import websockets
from websockets.exceptions import ConnectionClosedError 
import base64
from PIL import Image
import io
import os

async def handle_image(websocket):
	try: 
		async for message in websocket:
			try:
				image_data = base64.b64decode(message)
				img = Image.open(io.BytesIO(image_data))
				result = "Process success. Processed Image Size: {}x{}".format(img.width, img.height)
				await websocket.send(result)
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
		max_size=10 * 1024 * 1024,
		ping_interval=None,
		ping_timeout=None,
		close_timeout=10
	):
		await asyncio.Future()

if __name__ == "__main__":
	asyncio.run(main())