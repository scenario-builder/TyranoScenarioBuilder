var sidebarBtn = document.querySelector('.sidebar-toggle');
var sidebar = document.querySelector('.sidebar');

if(sidebarBtn && sidebar) {
  sidebarBtn.addEventListener('click', function() {
    sidebarBtn.classList.toggle('sidebar-closed');
    sidebar.classList.toggle('sidebar-closed');
  })
}