import asyncio
import websockets
import base64
from PIL import Image
import io

async def handle_image(websockets):
	async for message in websockets:
		image_data = base64.b64decode(message)
		img = Image.open(io.BytesIO(image_data))

		result = "Process success. Processed Image Size: {}x{}".format(img.width, img.height)

		await websockets.send(result)

async def main():
	async with websockets.serve(handle_image, "localhost", 8765):
		print("WebSocket server started on ws://localhost:8765")
		await asyncio.Future()  # run forever

if __name__ == "__main__":
    asyncio.run(main())