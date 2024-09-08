async function getProjects() {
  try {
      const response = await fetch('http://127.0.0.1:3030/projects');
      if (!response.ok) {
          throw new Error('Network response was not ok');
      }
      const projects = await response.json();
      const projectList = document.getElementById('project-list');

      projectList.innerHTML = projects.map(project => `
          <div class="project">
              <h3>${project.name}</h3>
              <p>Owner: ${project.owner}</p>
              <p>Target: ${project.target_amount}</p>
              <p>Current: ${project.current_amount}</p>
              <p>Funded: ${project.is_funded}</p>
          </div>
      `).join('');
  } catch (error) {
      console.error('Failed to fetch projects:', error);
  }
}

document.getElementById('create-project-form').addEventListener('submit', async (event) => {
  event.preventDefault();
  const name = document.getElementById('project-name').value;
  const owner = document.getElementById('project-owner').value;
  const targetAmount = document.getElementById('target-amount').value;

  try {
      const response = await fetch('http://127.0.0.1:3030/projects', {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json'
          },
          body: JSON.stringify({ name, owner, target_amount: targetAmount })
      });

      if (!response.ok) {
          throw new Error('Network response was not ok');
      }

      getProjects();
  } catch (error) {
      console.error('Failed to create project:', error);
  }
});

document.getElementById('transfer-funds-form').addEventListener('submit', async (event) => {
  event.preventDefault();
  const projectName = document.getElementById('transfer-project-name').value;

  try {
      const response = await fetch('http://127.0.0.1:3030/transfer', {
          method: 'POST',
          headers: {
              'Content-Type': 'application/json'
          },
          body: JSON.stringify({ project_name: projectName })
      });

      if (!response.ok) {
          throw new Error('Network response was not ok');
      }

      getProjects();
  } catch (error) {
      console.error('Failed to transfer funds:', error);
  }
});