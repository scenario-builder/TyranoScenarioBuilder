"use strict";
const sidebarBtn = document.querySelector('.sidebar-toggle');
const sidebar = document.querySelector('.sidebar');
if (sidebarBtn && sidebar) {
    sidebarBtn.addEventListener('click', function () {
        sidebarBtn.classList.toggle('sidebar-closed');
        sidebar.classList.toggle('sidebar-closed');
    });
}
//# sourceMappingURL=sidebar.js.map