#!/usr/bin/env python3
# This script is used to start/finish bootstrapping process.
# For more info see Bootstrapping.md in the rustemo book.
import os
import argparse
import subprocess

project_root = os.path.join(os.path.dirname(__file__), '..')
p_root = ['rustemo', 'src', 'lang']
parser_path = os.path.join(*p_root, 'rustemo.rs')
bootstrap_parser_path = os.path.join(*p_root, 'rustemo_bootstrap.rs')
actions_path = os.path.join(*p_root, 'rustemo_actions.rs')
bootstrap_actions_path = os.path.join(*p_root, 'rustemo_actions_bootstrap.rs')

def bootstrap_start():
    print('Checking out parser file.')
    status = subprocess.run(['git', 'show', f'main:{parser_path}'], capture_output=True)
    if status.returncode != 0:
        print(f'git show command returned error: {status.stderr}.')
        return

    print('Checking out actions file.')
    status_actions = subprocess.run(['git', 'show', f'main:{actions_path}'],
                                    capture_output=True)
    if status_actions.returncode != 0:
        print(f'git show command returned error: {status_actions.stderr}.')
        return

    with open(os.path.join(project_root, bootstrap_parser_path), 'bw') as f:
       f.write(status.stdout)

    with open(os.path.join(project_root, bootstrap_actions_path), 'bw') as f:
       f.write(status_actions.stdout)

    print('Git checkout complete!')
    print('Running "cargo build --features bootstrap" to produce bootstrap binary.')

    status = subprocess.run(['cargo', 'build', '--features', 'bootstrap'])
    if status.returncode != 0:
        print(f'cargo returned error: "{status.stderr}"')

    print('Bootstrapping initialization complete!')


def bootstrap_finish():
    print('Deleting bootstrap files.')

    for f in [bootstrap_parser_path, bootstrap_actions_path]:
        f = os.path.join(project_root, f)
        if os.path.exists(f):
            os.remove(f)
        else:
            print(f'File "{f}" doesn\'t exist.')

    print('Done.')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Rustemo bootstrapping script.'
                                    ' See Bootstrapping.md for more info')
    parser.add_argument('--start',  action='store_true', help='Starts the bootstrap process.')
    parser.add_argument('--finish', action='store_true', help='Finishes the bootstrap process.')

    args = parser.parse_args()
    if not (args.finish or args.start):
        parser.print_help()
    else:
        if args.start:
            bootstrap_start()
        else:
            bootstrap_finish()
