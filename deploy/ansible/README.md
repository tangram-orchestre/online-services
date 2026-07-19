# Ansible

This directory contains Ansible playbooks and roles for setting up the server environment for the Tangram online services. The playbooks automate the installation and configuration of necessary software, including Docker, and ensure that the server is ready to host the services.

## Prerequisites

- Ansible must be installed on the control machine.
- The target servers must be accessible via SSH and have Python installed.
- The user running the playbooks must have sudo privileges on the target servers.
- The target servers should have a supported Linux distribution (Debian).

## Usage

1. Clone the repository to your local machine.
2. Navigate to the `deploy/ansible` directory.
3. Install the required Ansible roles using the following command:
   ```bash
   ansible-galaxy install -r requirements.yml
   ```
4. Run the playbook to set up the servers:
   ```bash
   ansible-playbook -i inventory.yaml playbook.yaml
   ```
