def removeNthFromEnd(self, head: ListNode, n: int) -> ListNode:
    p = head
    q = head
    
    counter = 0
    
    while counter != n:
        q = q.next
        counter += 1
        
    if q is None:
        p = p.next
        return p
        
    while q.next is not None:
        p = p.next
        q = q.next
        
    p.next = p.next.next

    return head