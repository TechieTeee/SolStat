# SolStat
Solana Rust-based ETL Data Pipeline

## Table of Contents

1. [Introduction](#introduction)
2. [Tech Stack](#tech-stack)
3. [Purpose of SolStat](#purpose-of-solstat)
4. [Applications for Real-World Use Cases](#applications-for-real-world-use-cases)
5. [How SolStat Can Help Data Engineering](#how-solstat-can-help-data-engineering)
6. [Contribution](#contribution)
7. [Running the Script](#running-the-script)
8. [License](#license)

## Introduction

SolStat is an open-source Rust-based project designed to streamline the extraction, transformation, and loading (ETL) of data from the Solana blockchain. As Solana becomes a prominent platform for blockchain applications, a lack of dedicated data tools and comprehensive documentation poses a significant challenge for developers seeking to build data-driven applications. SolStat aims to address this gap by providing a versatile ETL tool with a focus on making it easier for developers to access and analyze Solana data.

## Tech Stack

SolStat is built using the following technologies:

- [Rust](https://www.rust-lang.org/): The core programming language used for the project.
- [Solana](https://solana.com/): The high-performance blockchain platform from which data is extracted.
- [Quicknode](https://www.quicknode.com/): The endpoint for accessing Solana data efficiently.

## Purpose of SolStat

- **Data Accessibility**: SolStat simplifies the extraction of data from the Solana blockchain, making it more accessible for developers.
- **Documentation**: Comprehensive documentation is provided to guide developers through the process of collecting, storing, and analyzing Solana data.
- **Inspiration**: SolStat aims to inspire new data-driven projects on Solana, fostering innovation in the blockchain space.

## Applications for Real-World Use Cases

SolStat can be applied to various real-world use cases, including:

- **Blockchain Analytics**: SolStat is an invaluable tool for analyzing on-chain data, transaction data, order book data, and more.
- **Data Products**: It can be used to develop new data products for financial services or market analysis.
- **Risk Management**: SolStat can aid in risk assessment and fraud detection in blockchain transactions.

## How SolStat Can Help Data Engineering

SolStat plays a crucial role in advancing data engineering projects on Solana and contributing to the growth of the Rust and Solana communities. It lowers the barriers for developers to build data-based applications on the Solana blockchain, facilitating innovation in areas such as data engineering, machine learning, generative AI, and more.

## Contribution

Feel free to make a contribution. To contribute to SolStat:

1. Fork the repository.
2. Make your changes and improvements.
3. Submit a pull request with a clear description of your changes.

## Running the Script

To run the SolStat script:

1. Clone the repository to your local machine.
2. Set up the required environment variables, including the Solana endpoint URL. You have to add your own .env file and then use your own Solana Quicknode endpoint. This can be done with the free tier of Quicknode.
3. Execute the script by running the following command:

```bash
$ cargo run
