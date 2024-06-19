from setuptools import setup

setup(
    name='reddit-motion',
    version='1.2',
    author='Aarav Shah',
    description='Command-line arguments parser for reddit-motion\'s external script function.',
    url='https://github.com/Deaths-Door/reddit-motion',
    python_requires='>=3.7, <4',
    install_requires=[
        'argparse>=1.4.0',
    ],
)