/**
 * 格式化日期时间
 * @param {string} dateStr 日期时间字符串
 * @returns {string} 格式化后的日期时间字符串
 */
export function formatDateTime(dateStr) {
  if (!dateStr) return '';
  
  // 如果是标准的日期时间字符串格式（YYYY-MM-DD HH:mm:ss），直接返回
  if (/^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$/.test(dateStr)) {
    return dateStr;
  }
  
  const date = new Date(dateStr);
  if (isNaN(date.getTime())) {
    console.warn('Invalid date:', dateStr);
    return '';
  }
  
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  const hours = String(date.getHours()).padStart(2, '0');
  const minutes = String(date.getMinutes()).padStart(2, '0');
  const seconds = String(date.getSeconds()).padStart(2, '0');
  return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
} 