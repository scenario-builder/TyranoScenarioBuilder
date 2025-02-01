const sidebarBtn = document.querySelector('.sidebar-toggle') as HTMLElement | null;
const sidebar = document.querySelector('.sidebar') as HTMLElement | null;

if(sidebarBtn && sidebar) {
  sidebarBtn.addEventListener('click', function() {
    sidebarBtn.classList.toggle('sidebar-closed');
    sidebar.classList.toggle('sidebar-closed');
  })
}