import emitter from './eventBus'

export const updateUserInfo = (user) => {
  const userInfo = {
    id: user.id,
    username: user.username,
    nickname: user.nickname,
    email: user.email,
    avatar: user.avatar,
    status: user.status,
    created_at: user.created_at,
    updated_at: user.updated_at
  }
  localStorage.setItem('user', JSON.stringify(userInfo))
  emitter.emit('user-updated', userInfo)
  return userInfo
} 