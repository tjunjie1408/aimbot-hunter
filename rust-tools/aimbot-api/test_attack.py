import requests
import json
import math
import random
import time

API_URL = "http://127.0.0.1:3002/analyze"

def generate_human_path():

    path = []
    t = 0
    for i in range(60):
        x = i * 5.0
        y = math.sin(i * 0.1) * 20.0 
        
        path.append({"x": x, "y": y})
    return path

def generate_aimbot_path():
    path = []
    x, y = 0, 0
    for i in range(60):
        path.append({"x": float(x), "y": float(y)})
        
        if i % 10 == 0:
            x += 200
            y += 200
        else:
            x += random.uniform(-1, 1)
            y += random.uniform(-1, 1)
            
    return path

def send_to_rust_brain(trajectory, name):
    payload = {"trajectory": trajectory}
    print(f"\n🚀 Sending [{name}] data to Rust Brain...")
    
    try:
        start_time = time.time()
        response = requests.post(API_URL, json=payload)
        end_time = time.time()
        
        result = response.json()
        latency = (end_time - start_time) * 1000 
        score = result['anomaly_score']
        
        print(f"⏱️  Latency: {latency:.2f} ms")
        print(f"📊 Score:   {score:.6f}")
        
        bar_len = min(50, int(score * 100))
        print(f"📈 Graph:   |{'█' * bar_len:<50}|")

        if score > 0.2: 
             print(f"🧠 Verdict: 🚨 CHEATER (Score > 0.2)")
        else:
             print(f"🧠 Verdict: ✅ HUMAN   (Score < 0.2)")

    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    print("=== 🛡️ AIMBOT HUNTER CALIBRATION ===")
    
    human_data = generate_human_path()
    send_to_rust_brain(human_data, "HUMAN (Smooth)")

    print("-" * 30)

    bot_data = generate_aimbot_path()
    send_to_rust_brain(bot_data, "AIMBOT (Teleport)")