import tensorflow as tf
import joblib
import tf2onnx
import json
import numpy as np
import os

MODEL_PATH = 'aimbot_hunter_model.h5'
SCALER_PATH = 'scaler.save'
ONNX_OUTPUT = '../rust-tools/aimbot-api/model.onnx'
CONFIG_OUTPUT = '../rust-tools/aimbot-api/config.json'

def export():
    print(">>> 1. Loading Keras Model...")
    model = tf.keras.models.load_model(MODEL_PATH, compile=False)
    
    if not hasattr(model, 'output_names'):
        model.output_names = ['output_0']
    
    print(">>> 2. Converting to ONNX (Open Standard)...")
    spec = (tf.TensorSpec((None, 50, 2), tf.float32, name="input"),)
    model_proto, _ = tf2onnx.convert.from_keras(model, input_signature=spec)
    
    with open(ONNX_OUTPUT, "wb") as f:
        f.write(model_proto.SerializeToString())
    print(f"✅ Model exported to {ONNX_OUTPUT}")

    print(">>> 3. Extracting Scaler Parameters...")
    scaler = joblib.load(SCALER_PATH)
    
    scaler_config = {
        "min": scaler.min_.tolist(),
        "scale": scaler.scale_.tolist(),
        "threshold": 0.01
    }
    
    with open(CONFIG_OUTPUT, "w") as f:
        json.dump(scaler_config, f, indent=4)
    print(f"✅ Config exported to {CONFIG_OUTPUT}")

if __name__ == "__main__":
    if not os.path.exists('../rust-tools/aimbot-api'):
        print("❌ Error: Please create the 'aimbot-api' rust project first!")
    else:
        export()