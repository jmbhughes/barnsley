import json
import subprocess
import numpy as np

def convex_combine(start, end, theta):
    return (theta * start + (1 - theta) * end)

if __name__ == "__main__":
    with open("start.json") as f:
        start_config = json.load(f)

    with open("end.json") as f:
        end_config = json.load(f)

    for i, theta in enumerate(np.linspace(1, 0.8, 100)):
        out_file_name = f"morph_movie3/{i:03d}.json"
        this_config = start_config.copy()
        this_config['image_settings']['path'] = out_file_name.replace(".json", ".png")

        for i, transform in enumerate(this_config['transforms']):
            name = list(transform.keys())[0]
            keys = transform[name].keys()
            for key in keys:
                if isinstance(transform[name][key], float):
                    start_value = start_config['transforms'][i][name][key]
                    end_value = end_config['transforms'][i][name][key]

                    this_config['transforms'][i][name][key] = convex_combine(start_value, 
                                                                             end_value, 
                                                                             theta)
                else:
                    if isinstance(transform[name][key], dict):
                        for inner_key in transform[name][key].keys():
                            if isinstance(transform[name][key][inner_key], float):
                                start_value = start_config['transforms'][i][name][key][inner_key]
                                end_value = end_config['transforms'][i][name][key][inner_key]

                                this_config['transforms'][i][name][key][inner_key] = convex_combine(start_value, 
                                                                                        end_value, 
                                                                                    theta)
                    else:
                         for j, value in enumerate(transform[name][key]):
                            if isinstance(value, float):
                                start_value = start_config['transforms'][i][name][key][j]
                                end_value = end_config['transforms'][i][name][key][j]

                                this_config['transforms'][i][name][key][j] = convex_combine(start_value, 
                                                                                        end_value, 
                                                                                    theta)
        # this_config['transforms'][0]['LinearTransform']['c'] = float(v)
        # this_config['transforms'][-1]['InverseJuliaTransform']['c'][0] = float(v)
        # this_config['transforms'][-1]['InverseJuliaTransform']['c'][1] = float(v)
        # this_config['transforms'][-1]['InverseJuliaTransform']['theta'] = float(v)
        # this_config['transforms'][-1]['InverseJuliaTransform']['weight'] = 1 - v/10
        # this_config['transforms'][-1]['InverseJuliaTransform']['base_color']['r'] = 1 - v/10
        # this_config['transforms'][-1]['InverseJuliaTransform']['base_color']['g'] = v/10


        with open(out_file_name, 'w') as f:
            json.dump(this_config, f)
        subprocess.run(['cargo', 'run', '--release', 'evaluate', out_file_name])