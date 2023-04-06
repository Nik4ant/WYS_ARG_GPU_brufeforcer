import os, subprocess
from typing import Tuple
from decimal import *


# :)
PLAY_INTALD_MUSIC = False
# TODO: this must be synced with main code, but I doubt it will change, so whatever...
KEYS_PER_INVOCATION = 32  # How much keys are tested by one thread
INTALD_MUSIC_PATH = "./gas_gas_gas.mp3"
EXECUTABLE_PATH = "../x64/Release/WYS_Cuda.exe"
# How much each configuration will be called
EXECUTIONS_PER_CONFIG = 10
# (THREADS, BLOCKS)
EXECUTION_CONFIGS = [
    (256, 1024),
    (512, 1024),
    (1024, 1024),
    (256, 1024 * 2),
    (512, 1024 * 2),
    (1024, 1024 * 2),
    (256, 1024 * 4),
    (512, 1024 * 4),
    (1024, 1024 * 4),
    (256, 1024 * 8),
    (512, 1024 * 8),
    (1024, 1024 * 8),
    (256, 1024 * 16),
    (512, 1024 * 16),
    (1024, 1024 * 16),
]
OUTPUT_DIR_PATH = "./results"


class NvprofInfo:
    # TODO:
    pass


class PerfomanceInfo:
    rough_avarage_ms: Decimal
    rough_min_ms: Decimal
    rough_max_ms: Decimal
    rough_keys_per_second: int

    threads: int
    blocks: int
    
    nvprof_info: NvprofInfo

    def __init__(self, threads: int, blocks: int):
        self.rough_avarage_ms = 0
        self.rough_min_ms = 0
        self.rough_max_ms = 0
        self.rough_keys_per_second = 0
        self.nvprof_info = NvprofInfo()
        
        self.threads = threads
        self.blocks = blocks

    def __str__(self):
        result = str(f"({str(self.threads)}, {self.blocks})").ljust(16)
        result += str(f"Min: {self.rough_min_ms} ms").ljust(20)
        result += str(f"Avarage: {self.rough_avarage_ms} ms").ljust(26)
        result += str(f"Max: {self.rough_max_ms} ms").ljust(20)
        result += str(f"Keys per sec: {self.rough_keys_per_second}").ljust(24)
        return result


def calculate_config_perfomance(benchmark_data_dir: str) -> PerfomanceInfo:
    config_raw: str = os.path.basename(benchmark_data_dir)
    used_config: Tuple[int, int] = tuple(map(int, config_raw.split()))
    
    result = PerfomanceInfo(used_config[0], used_config[1])
    
    rough_ellapsed_ms_sums = []
    
    for filename in os.listdir(benchmark_data_dir):
        full_path = os.path.join(benchmark_data_dir, filename)
        with open(full_path) as file:
            if filename.startswith("profiler_"):
                # TODO: NvprofInfo
                pass
            elif filename.startswith("stdout_"):
                stdout_data = file.readlines()
                elapsed_ms: Decimal = Decimal(stdout_data[2].split()[-1].rstrip("ms"))
                rough_ellapsed_ms_sums.append(elapsed_ms)
            else:
                raise Exception(f"ERROR! Can't parse `{filename}`; Unknown filename")
    
    result.rough_avarage_ms = sum(rough_ellapsed_ms_sums) / EXECUTIONS_PER_CONFIG
    result.rough_min_ms = min(rough_ellapsed_ms_sums)
    result.rough_max_ms = max(rough_ellapsed_ms_sums)
    result.rough_keys_per_second = round((used_config[0] * used_config[1] * KEYS_PER_INVOCATION) * result.rough_avarage_ms)

    return result


def get_folders_count(path: str) -> int:
    result = 0
    for name in os.listdir(path):
        if os.path.isdir(os.path.join(OUTPUT_DIR_PATH, name)):
            result += 1
    return result


def main():
    for folder in os.listdir("./results/0"):
        info: PerfomanceInfo = calculate_config_perfomance(f"./results/0/{folder}")
        print(info)
    return
    # Creating folder for current benchmark
    cur_folder_id: str = str(get_folders_count(OUTPUT_DIR_PATH))
    cur_folder_path: str = os.path.join(OUTPUT_DIR_PATH, cur_folder_id)
    os.mkdir(cur_folder_path)
    # Begin the benchmark
    for config in EXECUTION_CONFIGS:
        # Create dir for current config and run the app
        config_dir_path: str = os.path.join(cur_folder_path, f"{config[0]} {config[1]}")
        os.mkdir(config_dir_path)
        for i in range(EXECUTIONS_PER_CONFIG):
            profiler_output_path: str = os.path.join(config_dir_path, f"profiler_{i}.txt")
            output: str = subprocess.check_output([
                "nvprof", "--csv", "--log-file", profiler_output_path,
                EXECUTABLE_PATH, str(config[0]), str(config[1])
            ], universal_newlines='\n')
            # Writing stdout to the file for extra parsing
            stdout_output_path: str = os.path.join(config_dir_path, f"stdout_{i}.txt")
            with open(stdout_output_path, mode='w') as file:
                file.write(output)
            
    # input("Press ENTER to exit: ")
    


if __name__ == "__main__":
    if PLAY_INTALD_MUSIC:
        try:
            from playsound import playsound
            playsound(INTALD_MUSIC_PATH, False)
        except Exception as e:
            print(f"Can't play {INTALD_MUSIC_PATH} in the background; {e}")

    main()
